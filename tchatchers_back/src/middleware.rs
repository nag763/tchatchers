use crate::State;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use regex::Regex;
use std::sync::Arc;
use tchatchers_core::jwt::Jwt;

lazy_static! {
    static ref JWT_COOKIE_HEADER: Regex =
        Regex::new(r####"^jwt=(?P<token_val>[a-zA-Z0-9\._-]*)$"####).unwrap();
}

pub async fn secure_route<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, impl IntoResponse> {
    let headers = req.headers();
    if let Some(cookie) = headers.get("cookie") {
        let cookie_val = cookie.to_str().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't find cookie value",
            )
        })?;
        let captures = JWT_COOKIE_HEADER.captures(cookie_val).ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Regex couldn't find the cookie value",
        ))?;
        let value: &str = captures.name("token_val").unwrap().as_str();
        match Jwt::deserialize(
            value,
            &req.extensions().get::<Arc<State>>().unwrap().jwt_secret,
        ) {
            Ok(_) => Ok(next.run(req).await),
            Err(_) => Err((StatusCode::UNAUTHORIZED, "JWT invalid")),
        }
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            "The route is only for authorized users, please log in.",
        ))
    }
}
