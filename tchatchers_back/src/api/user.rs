//! Gathers all the API used to do CRUD operations on user entity.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::extractor::JwtUserExtractor;
use crate::validator::ValidJson;
use crate::AppState;
use crate::REFRESH_TOKEN_PATH;
use axum::extract::State;
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use tchatchers_core::authorization_token::AuthorizationToken;
use tchatchers_core::refresh_token::RefreshToken;
use tchatchers_core::serializable_token::SerializableToken;
use tchatchers_core::user::{AuthenticableUser, InsertableUser, UpdatableUser, User};
use tokio::time::{sleep, Duration};
use tracing::log::error;

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
    ValidJson(new_user): ValidJson<InsertableUser>,
) -> impl IntoResponse {
    if User::login_exists(&new_user.login, &state.pg_pool).await {
        return Err((
            StatusCode::BAD_REQUEST,
            "A user with a similar login already exists",
        ));
    }

    match new_user.insert(&state.pg_pool).await {
        Ok(_) => Ok((StatusCode::CREATED, "User created with success")),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "An error happened")),
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
) -> impl IntoResponse {
    match User::login_exists(&login, &state.pg_pool).await {
        false => StatusCode::OK,
        true => StatusCode::CONFLICT,
    };
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
    ValidJson(authenticable_user): ValidJson<AuthenticableUser>,
) -> impl IntoResponse {
    let Some(user) = authenticable_user.authenticate(&state.pg_pool).await else {
            sleep(Duration::from_secs(3)).await;
            return Err((StatusCode::NOT_FOUND, "We couldn't connect you, please ensure that the login and password are correct before trying again"));
    };
    if user.is_authorized {
        let refresh_token = {
            let mut redis_conn = state.redis_pool.get();
            let redis_conn_unwrapped = redis_conn.as_deref_mut().unwrap();

            let token = RefreshToken::new(user.id, authenticable_user.session_only);
            token.set_as_head_token(redis_conn_unwrapped);
            token
        };
        let jwt: AuthorizationToken = AuthorizationToken::from(user);
        Ok((
            StatusCode::OK,
            refresh_token
                .store_in_jar(&state.refresh_token_secret, cookie_jar)
                .unwrap(),
            jwt.encode(&state.jwt_secret).unwrap(),
        ))
    } else {
        Err((StatusCode::UNAUTHORIZED, "This user's access has been revoked, contact an admin if you believe you should access this service"))
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
        return Err((StatusCode::BAD_REQUEST, "Your session has expired. Please log in again."))
    };

    // Decode the refresh token and verify that it is legitimate.
    let Ok(refresh_token) = RefreshToken::decode(cookie.value(), &state.refresh_token_secret) else {
        return Err((StatusCode::BAD_REQUEST, "Your authentication token is expired or illegitimate. Please log in again."))
    };

    // Refresh the token.
    let refreshed_token = {
        // Get a Redis connection from the Redis connection pool.
        let mut redis_conn = state.redis_pool.get();
        let redis_conn_unwrapped = redis_conn.as_deref_mut().unwrap();

        if !refresh_token.is_head_token(redis_conn_unwrapped) {
            refresh_token.revoke_family(redis_conn_unwrapped);
            return Err((
                StatusCode::UNAUTHORIZED,
                "There was an issue while refreshing your session. Please log in again.",
            ));
        } else {
            let refreshed_token = refresh_token.renew();
            refreshed_token.set_as_head_token(redis_conn_unwrapped);
            refreshed_token
        }
    };

    // Retrieve the user corresponding to the refresh token's user ID from the database.
    let Some(user) = User::find_by_id(refresh_token.user_id, &state.pg_pool).await else {
        return Err((StatusCode::NOT_FOUND, "Your account hasn't been found back, please log in again."))
    };

    // Verify that the user's account is authorized.
    if !user.is_authorized {
        return Err((
            StatusCode::FORBIDDEN,
            "Your account has been deactivated. Please log out.",
        ));
    }
    let encoded_jwt: String = AuthorizationToken::from(user)
        .encode(&state.jwt_secret)
        .unwrap();

    // Renew the refresh token and store the updated value in the cookie jar.
    Ok((
        StatusCode::OK,
        refreshed_token
            .store_in_jar(&state.refresh_token_secret, cookie_jar)
            .unwrap(),
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
pub async fn logout(State(state): State<AppState>, cookie_jar: CookieJar) -> impl IntoResponse {
    // Attempt to retrieve the refresh token from the cookie jar.
    if let Some(cookie) = cookie_jar.get(REFRESH_TOKEN_PATH) {
        if let Ok(refresh_token) = RefreshToken::decode(cookie.value(), &state.refresh_token_secret)
        {
            {
                // Get a Redis connection from the Redis connection pool.
                let mut redis_conn = state.redis_pool.get();
                let redis_conn_unwrapped = redis_conn.as_deref_mut().unwrap();

                refresh_token.revoke_family(redis_conn_unwrapped);
            }
        }
    }

    let mut cookie = Cookie::named(REFRESH_TOKEN_PATH);
    cookie.set_path("/");
    let new_jar = cookie_jar.remove(cookie);
    (StatusCode::OK, new_jar).into_response()
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
    ValidJson(user): ValidJson<UpdatableUser>,
) -> impl IntoResponse {
    if jwt.user_id == user.id {
        if let Err(err) = user.update(&state.pg_pool).await {
            error!("An error happened while trying to update the record : \n---New record :{:#?}---\nError : {}", user, err);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "An error happened"));
        };

        Ok((StatusCode::OK, "User updated with success"))
    } else {
        Err((StatusCode::UNAUTHORIZED, "You can't update other users"))
    }
}

/// Deletes a user from the database.
///
/// Only the user that requests this endpoint can delete himself.
pub async fn delete_user(
    JwtUserExtractor(jwt): JwtUserExtractor,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match User::delete_one(jwt.user_id, &state.pg_pool).await {
        Ok(_) => Ok((StatusCode::OK, "User updated with success")),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "An error happened")),
    }
}
