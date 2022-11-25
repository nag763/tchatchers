//! Defines the extractors used by the different webservices.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::AppState;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use regex::Regex;
use std::sync::Arc;
use tchatchers_core::jwt::Jwt;

lazy_static! {
    static ref JWT_COOKIE_HEADER: Regex =
        Regex::new(r####"jwt=(?P<token_val>[a-zA-Z0-9\._-]*)"####).unwrap();
}

/// Extracts the JWT from the request.
///
/// The JWT should be sent as a cookie to the server.
pub struct JwtUserExtractor(pub Jwt);

#[async_trait]
impl FromRequestParts<Arc<AppState>> for JwtUserExtractor {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
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
            match Jwt::deserialize(value, &state.jwt_secret) {
                Ok(v) => Ok(JwtUserExtractor(v)),
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
