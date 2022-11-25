//! Defines the extractors used by the different webservices.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::{AppState, JWT_PATH};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::CookieJar;
use std::sync::Arc;
use tchatchers_core::jwt::Jwt;

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
        let cookie_jar: CookieJar = CookieJar::from_headers(headers);
        if let Some(cookie) = cookie_jar.get(JWT_PATH) {
            match Jwt::deserialize(cookie.value(), &state.jwt_secret) {
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
