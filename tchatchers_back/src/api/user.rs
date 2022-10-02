use crate::State;
use crate::JWT_PATH;
use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, response::Redirect, Extension, Json,
};
use magic_crypt::MagicCryptTrait;
use std::sync::Arc;
use tchatchers_core::jwt::Jwt;
use tchatchers_core::user::{AuthenticableUser, InsertableUser, UpdatableUser, User};
use tokio::time::{sleep, Duration};
use tower_cookies::{Cookie, Cookies};

pub async fn create_user(
    Json(mut new_user): Json<InsertableUser>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    if User::login_exists(&new_user.login, &state.pg_pool).await {
        return (
            StatusCode::BAD_REQUEST,
            "A user with a similar login already exists",
        );
    }

    new_user.password = state.encrypter.encrypt_str_to_base64(&new_user.password);

    match new_user.insert(&state.pg_pool).await {
        Ok(_) => (StatusCode::CREATED, "User created with success"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "An error happened"),
    }
}

pub async fn login_exists(
    Path(login): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    let response_status: StatusCode = match User::login_exists(&login, &state.pg_pool).await {
        false => StatusCode::OK,
        true => StatusCode::CONFLICT,
    };
    (response_status, ())
}

pub async fn authenticate(
    Json(mut user): Json<AuthenticableUser>,
    Extension(state): Extension<Arc<State>>,
    cookies: Cookies,
) -> impl IntoResponse {
    user.password = state.encrypter.encrypt_str_to_base64(&user.password);
    let user = match user.authenticate(&state.pg_pool).await {
        Some(v) => v,
        None => {
            sleep(Duration::from_secs(3)).await;
            return (StatusCode::NOT_FOUND, "We couldn't connect you, please ensure that the login and password are correct before trying again");
        }
    };
    match user.is_authorized {
        true => {
            let jwt = Jwt::from(user);
            let serialized_jwt : String = jwt.serialize(&state.jwt_secret).unwrap();
            let mut jwt_cookie = Cookie::new(JWT_PATH, serialized_jwt);
            jwt_cookie.set_path("/");
            jwt_cookie.make_permanent();
            jwt_cookie.set_secure(true);
            jwt_cookie.set_http_only(false);
            cookies.add(jwt_cookie);
            (StatusCode::OK, "")
        }
        false => (StatusCode::UNAUTHORIZED, "This user's access has been revoked, contact an admin if you believe you should access this service")
    }
}

pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    let mut jwt_cookie = Cookie::new(JWT_PATH, "");
    jwt_cookie.set_path("/");
    jwt_cookie.make_removal();
    cookies.add(jwt_cookie);
    (StatusCode::OK, "")
}

pub async fn validate(
    cookies: Cookies,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    if let Some(cookie) = cookies.get(JWT_PATH) {
        let value = cookie.value();
        match Jwt::deserialize(value, &state.jwt_secret) {
            Ok(_) => (StatusCode::OK, ""),
            Err(_) => (StatusCode::UNAUTHORIZED, "The jwt couldn't be deserialized"),
        }
    } else {
        (StatusCode::UNAUTHORIZED, "The JWT token hasn't been found")
    }
}

pub async fn update_user(
    Json(user): Json<UpdatableUser>,
    Extension(state): Extension<Arc<State>>,
    cookies: Cookies,
) -> impl IntoResponse {
    if let Some(cookie) = cookies.get(JWT_PATH) {
        if let Ok(jwt) = Jwt::deserialize(&cookie.value(), &state.jwt_secret) {
            if jwt.user.id == user.id {
                match user.update(&state.pg_pool).await {
                    Ok(_) => {
                        let updated_user = User::find_by_id(user.id, &state.pg_pool).await.unwrap();
                        let jwt = Jwt::from(updated_user);
                        let serialized_jwt: String = jwt.serialize(&state.jwt_secret).unwrap();
                        let mut jwt_cookie = Cookie::new(JWT_PATH, serialized_jwt);
                        jwt_cookie.set_path("/");
                        jwt_cookie.make_permanent();
                        jwt_cookie.set_secure(true);
                        jwt_cookie.set_http_only(false);
                        cookies.add(jwt_cookie);
                        (StatusCode::CREATED, "User updated with success").into_response()
                    }
                    Err(_) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, "An error happened").into_response()
                    }
                }
            } else {
                (StatusCode::FORBIDDEN, "You can't update another user").into_response()
            }
        } else {
            Redirect::to("/logout").into_response()
        }
    } else {
        (StatusCode::UNAUTHORIZED, "This route is protected").into_response()
    }
}
