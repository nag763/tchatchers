//! Gathers all the API used to do CRUD operations on user entity.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::extractor::JwtUserExtractor;
use crate::extractor::ModeratorExtractor;
use crate::extractor::Postcard;
use crate::extractor::ValidPostcard;
use crate::AppState;
use crate::REFRESH_TOKEN_PATH;
use axum::extract::Multipart;
use axum::extract::State;
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use tchatchers_core::api_response::ApiGenericResponse;
use tchatchers_core::async_message::AsyncMessage;
use tchatchers_core::authorization_token::AuthorizationToken;
use tchatchers_core::mail::mail::Mail;
use tchatchers_core::mail::template::WelcomeMailContent;
use tchatchers_core::refresh_token::RefreshToken;
use tchatchers_core::report::Report;
use tchatchers_core::serializable_token::SerializableToken;
use tchatchers_core::user::PartialUser;
use tchatchers_core::user::{AuthenticableUser, InsertableUser, UpdatableUser, User};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use validator::Validate;

/// Creates a user.
///
/// The password will be encrypted server side.
///
/// # Arguments
///
/// - new_user : The user to insert in database.
/// - state : The data shared across thread.
pub async fn create_user(
    State(state): State<AppState>,
    ValidPostcard(new_user): ValidPostcard<InsertableUser>,
) -> impl IntoResponse {
    if User::login_exists(&new_user.login, &state.pg_pool).await? {
        return Err(ApiGenericResponse::SimilarLoginExists);
    }

    if let Some(mail) = &new_user.email {
        if User::email_exists(mail, &state.pg_pool).await? {
            return Err(ApiGenericResponse::SimilarEmailExists);
        }
    }

    new_user.insert(&state.pg_pool).await?;
    if state.mails_enabled {
        if let Some(email) = new_user.email {
            if let Some(mail) = Mail::new(
                email,
                WelcomeMailContent {
                    name: new_user.name,
                },
            ) {
                mail.send()
                    .await
                    .map_err(|_e| ApiGenericResponse::MailingError)?;
            }
        }
        Ok(ApiGenericResponse::CreationMailSent)
    } else {
        Ok(ApiGenericResponse::UserCreated)
    }
}

/// Check whether a login exists or not.
///
/// Useful when it is needed to create a new user for instance.
///
/// # Arguments
///
/// - login : The login to check.
/// - state : The data shared across thread.
pub async fn login_exists(
    Path(login): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiGenericResponse> {
    match User::login_exists(&login, &state.pg_pool).await? {
        false => Ok(StatusCode::OK),
        true => Err(ApiGenericResponse::SimilarLoginExists),
    }
}

/// Authenticate a user.
///
/// If the call to the service is successful, an authentication cookie will be
/// added to the user's browser.
///
/// # Arguments
/// - user : The user to authenticate.
/// - state : The data shared across thread.
/// - cookie_jar : The user's cookies.
pub async fn authenticate(
    cookie_jar: CookieJar,
    State(state): State<AppState>,
    ValidPostcard(authenticable_user): ValidPostcard<AuthenticableUser>,
) -> impl IntoResponse {
    let Some(user) = authenticable_user.authenticate(&state.pg_pool).await? else {
        sleep(Duration::from_secs(3)).await;
        return Err(ApiGenericResponse::BadCredentials);
    };
    if user.is_authorized {
        let refresh_token = {
            let mut redis_conn = state.session_pool.get().await?;

            let token = RefreshToken::new(user.id, authenticable_user.session_only);
            token.set_as_head_token(&mut redis_conn).await?;
            token
        };
        std::mem::drop(tokio::spawn(async move {
            let mut redis_conn = state.async_pool.get().await?;
            AsyncMessage::LoggedUser(user.id)
                .spawn(&mut redis_conn)
                .await;
            anyhow::Ok(())
        }));
        let jwt: AuthorizationToken = AuthorizationToken::from(user);
        Ok((
            StatusCode::OK,
            refresh_token.store_in_jar(&state.refresh_token_secret, cookie_jar)?,
            jwt.encode(&state.jwt_secret)?,
        ))
    } else {
        Err(ApiGenericResponse::AccessRevoked)
    }
}

/// Attempts to reauthenticate the user by verifying the refresh token stored in the provided `cookie_jar`.
/// If the refresh token is valid and not expired, a new authorization token is generated and returned along
/// with the updated refresh token. Otherwise, an error message is returned indicating the reason for the failure.
///
/// # Arguments
///
/// * `State(state)`: The state of the application.
/// * `cookie_jar`: The cookie jar containing the refresh token.
///
/// # Errors
///
/// Returns an error tuple `(StatusCode, &str)` if any of the following conditions are met:
/// * The refresh token is not found in the cookie jar.
/// * The refresh token is expired or illegitimate.
/// * The user corresponding to the refresh token's user ID is not found in the database.
/// * The user's account is not authorized.
///
/// # Return value
///
/// Returns a tuple `(StatusCode, String, String)` representing the HTTP response code, the updated refresh token,
/// and the new authorization token, respectively.
pub async fn reauthenticate(
    State(state): State<AppState>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    // Attempt to retrieve the refresh token from the cookie jar.
    let Some(cookie) = cookie_jar.get(REFRESH_TOKEN_PATH) else {
        return Err(ApiGenericResponse::AuthenticationExpired);
    };

    // Decode the refresh token and verify that it is legitimate.
    let Ok(refresh_token) = RefreshToken::decode(cookie.value(), &state.refresh_token_secret)
    else {
        return Err(ApiGenericResponse::AuthenticationRequired);
    };

    // Refresh the token.
    let refreshed_token = {
        // Get a Redis connection from the Redis connection pool.
        let mut redis_conn = state.session_pool.get().await?;

        if !refresh_token.is_head_token(&mut redis_conn).await? {
            refresh_token.revoke_family(&mut redis_conn).await?;
            return Err(ApiGenericResponse::AuthenticationExpired);
        } else {
            let refreshed_token = refresh_token.renew();
            refreshed_token.set_as_head_token(&mut redis_conn).await?;
            refreshed_token
        }
    };

    // Retrieve the user corresponding to the refresh token's user ID from the database.
    let Some(user) = User::find_by_id(refresh_token.user_id, &state.pg_pool).await? else {
        return Err(ApiGenericResponse::AccountNotFound);
    };

    // Verify that the user's account is authorized.
    if !user.is_authorized {
        return Err(ApiGenericResponse::AccessRevoked);
    }

    // Queue the information that user reauthenticated.
    std::mem::drop(tokio::spawn(async move {
        let mut redis_conn = state.async_pool.get().await?;
        AsyncMessage::LoggedUser(user.id)
            .spawn(&mut redis_conn)
            .await;
        anyhow::Ok(())
    }));

    let encoded_jwt: String = AuthorizationToken::from(user).encode(&state.jwt_secret)?;

    // Renew the refresh token and store the updated value in the cookie jar.
    Ok((
        StatusCode::OK,
        refreshed_token.store_in_jar(&state.refresh_token_secret, cookie_jar)?,
        encoded_jwt,
    ))
}

/// Log the user out.
///
/// This will erase the cookie from the user's browser.
///
/// # Arguments
///
/// - cookie_jar : The user's cookies.
pub async fn logout(
    State(state): State<AppState>,
    cookie_jar: CookieJar,
) -> Result<impl IntoResponse, ApiGenericResponse> {
    // Attempt to retrieve the refresh token from the cookie jar.
    if let Some(cookie) = cookie_jar.get(REFRESH_TOKEN_PATH) {
        if let Ok(refresh_token) = RefreshToken::decode(cookie.value(), &state.refresh_token_secret)
        {
            // Get a Redis connection from the Redis connection pool.
            let mut redis_conn = state.session_pool.get().await?;

            refresh_token.revoke_family(&mut redis_conn).await?;
        }
    }

    let cookie = Cookie::build(REFRESH_TOKEN_PATH).path("/");
    let new_jar = cookie_jar.remove(cookie);
    Ok((StatusCode::OK, new_jar).into_response())
}

/// Checks whether the authentication is legit, or if the user is authenticated
/// or not.
///
/// # Arguments
///
/// - jwt : The user's authentication token.
pub async fn validate(_: JwtUserExtractor) -> impl IntoResponse {
    StatusCode::OK
}

/// Update the user's informations.
///
/// There is a check server side to ensure that the user is only able to update
/// himself.
///
/// # Arguments
/// - jwt : The user authentication token.
/// - user : the new informations to update the user.
/// - state : The data shared across thread.
/// - cookie_jar : The user's cookies.
pub async fn update_user(
    JwtUserExtractor(jwt): JwtUserExtractor,
    State(state): State<AppState>,
    mut data: Multipart,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let payload = data.next_field().await?;
    let Some(payload) = payload else {
        return Err(ApiGenericResponse::SerializationError(
            "Multipart request with no field".into(),
        ));
    };
    let payload_bytes = payload.bytes().await?;
    let user: UpdatableUser = postcard::from_bytes(&payload_bytes)?;
    user.validate()?;

    if let Some(mail) = &user.email {
        if User::email_exists_except_self(mail, user.id, &state.pg_pool).await? {
            return Err(ApiGenericResponse::SimilarEmailExists);
        }
    }

    let mut tasks: JoinSet<Result<(), ApiGenericResponse>> = JoinSet::new();
    if jwt.user_id != user.id {
        return Err(ApiGenericResponse::UnsifficentPriviledges);
    }
    let Some(former_user) = PartialUser::find_by_id(jwt.user_id, &state.pg_pool).await? else {
        return Err(ApiGenericResponse::UserNotFound);
    };
    tasks.spawn({
        let user = user.clone();
        let pool = state.pg_pool.clone();
        async move {
            user.update(&pool).await?;
            Ok(())
        }
    });
    if let Some(file) = data.next_field().await.unwrap_or(None) {
        if let Some(_file_name) = file.file_name() {
            let bytes = file.bytes().await?;
            tasks.spawn(async move {
                let file_name = Uuid::new_v4().to_string();
                let rel_file_path = format!("./static/{}", file_name);
                let served_file_path: String = format!("/static/{}", file_name);
                let mut file = File::create(&rel_file_path).await?;
                file.write_all(&bytes).await?;
                UpdatableUser::set_pfp(user.id, &served_file_path, &state.pg_pool).await?;
                println!("Ok through");
                Ok(())
            });

            if former_user.pfp.is_some() {
                tasks.spawn(async move {
                    let mut redis_conn = state.async_pool.get().await?;
                    AsyncMessage::RemoveUserData(former_user)
                        .spawn(&mut redis_conn)
                        .await;
                    Ok(())
                });
            } else {
                std::mem::drop(former_user);
            }
        }
    }
    while let Some(task) = tasks.join_next().await {
        task.unwrap()?;
    }
    Ok((StatusCode::OK, "User updated with success"))
}

/// Deletes a user from the database.
///
/// Only the user that requests this endpoint can delete himself.
pub async fn delete_user(
    JwtUserExtractor(jwt): JwtUserExtractor,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiGenericResponse> {
    let Some(user) = PartialUser::find_by_id(jwt.user_id, &state.pg_pool).await? else {
        return Err(ApiGenericResponse::UserNotFound);
    };
    std::mem::drop(tokio::spawn(async move {
        let mut redis_conn = state.async_pool.get().await?;
        AsyncMessage::RemoveUserData(user)
            .spawn(&mut redis_conn)
            .await;
        anyhow::Ok(())
    }));
    User::delete_one(jwt.user_id, &state.pg_pool).await?;
    Ok(StatusCode::OK)
}

/// Revokes a user's access.
///
/// Only the user that requests this endpoint can delete himself.
pub async fn revoke_user(
    Path(user_id): Path<i32>,
    ModeratorExtractor(_): ModeratorExtractor,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiGenericResponse> {
    User::update_activation_status(user_id, false, &state.pg_pool).await?;
    Ok(ApiGenericResponse::RevokedUser)
}

/// Report a user.
///
/// # Arguments
///
/// - `reported_user`: The ID of the user to report.
///
/// This function reports a user by inserting a new entry in the `REPORTED_USER` table of the database. The reported user is associated with the user who made the request, as identified by the JWT token extracted from the request.
///
/// If the insertion is successful, the function returns a tuple containing the status code `StatusCode::OK` and a string indicating that the user has been reported.
///
/// If an error occurs during the insertion, the function checks if the error is a database error. If the error code is `23505`, it means that the user has already been reported, and the function returns a tuple with the status code `StatusCode::BAD_REQUEST` and a corresponding error message. Otherwise, it returns a tuple with the status code `StatusCode::INTERNAL_SERVER_ERROR` and a generic error message indicating that an error occurred while reporting the user.
pub async fn report_user(
    JwtUserExtractor(user): JwtUserExtractor,
    Path(reported_user): Path<i32>,
    state: State<AppState>,
) -> impl IntoResponse {
    match Report::user(user.user_id, reported_user, &state.pg_pool).await {
        Ok(_) => Ok(ApiGenericResponse::UserReported),
        Err(e) => {
            if let Some(database_err) = e.as_database_error() {
                if let Some(code) = database_err.code() {
                    if code == "23505" {
                        return Err(ApiGenericResponse::UserAlreadyReported);
                    }
                }
            }
            Err(ApiGenericResponse::DbError(e.to_string()))
        }
    }
}

pub async fn whoami(
    JwtUserExtractor(jwt): JwtUserExtractor,
    state: State<AppState>,
) -> Result<Postcard<PartialUser>, ApiGenericResponse> {
    let Some(user) = User::find_by_id(jwt.user_id, &state.pg_pool).await? else {
        return Err(ApiGenericResponse::UserNotFound);
    };
    if !user.is_authorized {
        return Err(ApiGenericResponse::AccessRevoked);
    }
    Ok(Postcard(user.into()))
}
