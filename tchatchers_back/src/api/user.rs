//! Gathers all the API used to do CRUD operations on user entity.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::extractor::JwtUserExtractor;
use crate::validator::Json;
use crate::AppState;
use crate::JWT_PATH;
use axum::extract::State;
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use std::sync::Arc;
use tchatchers_core::jwt::Jwt;
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
    State(state): State<Arc<AppState>>,
    Json(new_user): Json<InsertableUser>,
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
    State(state): State<Arc<AppState>>,
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
    State(state): State<Arc<AppState>>,
    Json(user): Json<AuthenticableUser>,
) -> impl IntoResponse {
    let Some(user) = user.authenticate(&state.pg_pool).await else {
            sleep(Duration::from_secs(3)).await;
            return Err((StatusCode::NOT_FOUND, "We couldn't connect you, please ensure that the login and password are correct before trying again"));
    };
    if user.is_authorized {
        let jwt = Jwt::from(user);
        let serialized_jwt: String = jwt.serialize(&state.jwt_secret).unwrap();
        let mut jwt_cookie = Cookie::new(JWT_PATH, serialized_jwt);
        jwt_cookie.set_path("/");
        jwt_cookie.make_permanent();
        jwt_cookie.set_secure(true);
        jwt_cookie.set_http_only(true);
        let cookie_jar = cookie_jar.add(jwt_cookie);
        Ok((StatusCode::OK, cookie_jar))
    } else {
        Err((StatusCode::UNAUTHORIZED, "This user's access has been revoked, contact an admin if you believe you should access this service"))
    }
}

/// Log the user out.
///
/// This will erase the cookie from the user's browser.
///
/// # Arguments
///
/// - cookie_jar : The user's cookies.
pub async fn logout(cookie_jar: CookieJar) -> impl IntoResponse {
    let mut cookie = Cookie::named(JWT_PATH);
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
pub async fn validate(jwt: Option<JwtUserExtractor>) -> impl IntoResponse {
    match jwt {
        Some(_) => Ok((StatusCode::OK, "")),
        None => Err((StatusCode::UNAUTHORIZED, "You aren't logged in.")),
    }
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
    State(state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
    Json(user): Json<UpdatableUser>,
) -> impl IntoResponse {
    if jwt.user.id == user.id {
        if let Err(err) = user.update(&state.pg_pool).await {
            error!("An error happened while trying to update the record : \n---New record :{:#?}---\nError : {}", user, err);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "An error happened"));
        };
        let updated_user = User::find_by_id(user.id, &state.pg_pool).await.unwrap();
        let jwt = Jwt::from(updated_user);
        let serialized_jwt: String = jwt.serialize(&state.jwt_secret).unwrap();
        let mut jwt_cookie = Cookie::new(JWT_PATH, serialized_jwt);
        jwt_cookie.set_path("/");
        jwt_cookie.make_permanent();
        jwt_cookie.set_secure(true);
        jwt_cookie.set_http_only(true);
        let new_jar = cookie_jar.add(jwt_cookie);
        Ok((StatusCode::CREATED, new_jar, "User updated with success"))
    } else {
        Err((StatusCode::UNAUTHORIZED, "You can't update other users"))
    }
}

/// Deletes a user from the database.
/// 
/// Only the user that requests this endpoint can delete himself. 
pub async fn delete_user(
    JwtUserExtractor(jwt): JwtUserExtractor,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match User::delete_one(jwt.user.id, &state.pg_pool).await {
        Ok(_) => Ok((StatusCode::OK, "User updated with success")),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "An error happened")),
    }
}
