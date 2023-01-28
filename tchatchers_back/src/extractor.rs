// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).


//! Defines the extractors used by the different webservices.

use crate::{AppState, JWT_PATH};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::CookieJar;
use std::sync::Arc;
use tchatchers_core::{jwt::Jwt, profile::Profile, user::PartialUser};

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
        let Some(cookie) = cookie_jar.get(JWT_PATH)  else {
            return Err((
                StatusCode::UNAUTHORIZED,
                "This route requires authentication",
            ));
        };
        match Jwt::deserialize(cookie.value(), &state.jwt_secret) {
            Ok(v) => Ok(JwtUserExtractor(v)),
            Err(_) => Err((StatusCode::UNAUTHORIZED, "JWT invalid")),
        }
    }
}

/// Extractor used to check that :
/// 
/// 1. The user is authenticated.
/// 2. The user has at least moderator roles in database.
pub struct ModeratorExtractor(pub PartialUser);

#[async_trait]
impl FromRequestParts<Arc<AppState>> for ModeratorExtractor {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let user = JwtUserExtractor::from_request_parts(parts, state)
            .await?
            .0
            .user;
        let user_profile: i32 = user.profile as i32;
        if user_profile < (Profile::Moderator as i32) {
            Err((
                StatusCode::UNAUTHORIZED,
                "You don't have sufficient privileges",
            ))
        } else {
            Ok(ModeratorExtractor(user))
        }
    }
}

/// Extractor used to check that :
/// 
/// 1. The user is authenticated.
/// 2. The user has at least admin roles in database.
pub struct AdminExtractor(pub PartialUser);

#[async_trait]
impl FromRequestParts<Arc<AppState>> for AdminExtractor {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let user = JwtUserExtractor::from_request_parts(parts, state)
            .await?
            .0
            .user;
        let user_profile: i32 = user.profile as i32;
        if user_profile < (Profile::Admin as i32) {
            Err((
                StatusCode::UNAUTHORIZED,
                "You don't have sufficient privileges",
            ))
        } else {
            Ok(AdminExtractor(user))
        }
    }
}
