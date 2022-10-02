use crate::State;
use axum::{async_trait, extract::FromRequest, extract::RequestParts, http::StatusCode};
use regex::Regex;
use std::sync::Arc;
use tchatchers_core::jwt::Jwt;

lazy_static! {
    static ref JWT_COOKIE_HEADER: Regex =
        Regex::new(r####"^jwt=(?P<token_val>[a-zA-Z0-9\._-]*)$"####).unwrap();
}

pub struct JwtUserExtractor {
    pub jwt: Jwt,
}

#[async_trait]
impl<B> FromRequest<B> for JwtUserExtractor
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        if let Some(cookie) = headers.get("cookie") {
            let cookie_val = cookie.to_str().map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Couldn't find cookie value",
                )
            })?;
            let captures = JWT_COOKIE_HEADER.captures(cookie_val).ok_or((
                StatusCode::UNAUTHORIZED,
                "You are not logged in, please log in prior accessing this service.",
            ))?;
            let value: &str = captures.name("token_val").unwrap().as_str();
            match Jwt::deserialize(
                value,
                &req.extensions().get::<Arc<State>>().unwrap().jwt_secret,
            ) {
                Ok(v) => Ok(JwtUserExtractor { jwt: v }),
                Err(_) => Err((StatusCode::UNAUTHORIZED, "JWT invalid")),
            }
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                "This route requires authentication",
            ))
        }
    }
}
