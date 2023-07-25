// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! Defines the extractors used by the different webservices.

use crate::AppState;
use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    TypedHeader,
};
use tchatchers_core::{
    api_response::ApiGenericResponse, authorization_token::AuthorizationToken, profile::Profile,
    serializable_token::SerializableToken,
};

/// Extracts the JWT from the request.
///
/// The JWT should be sent as a cookie to the server.
pub struct JwtUserExtractor(pub AuthorizationToken);

#[async_trait]
impl FromRequestParts<AppState> for JwtUserExtractor {
    type Rejection = ApiGenericResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Ok(TypedHeader(Authorization(jwt))) =  TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await else {
            return Err(ApiGenericResponse::AuthenticationRequired);
        };
        match AuthorizationToken::decode(jwt.token(), &state.jwt_secret) {
            Ok(v) => Ok(JwtUserExtractor(v)),
            Err(_) => Err(ApiGenericResponse::AuthenticationExpired),
        }
    }
}

/// Extractor used to check that :
///
/// 1. The user is authenticated.
/// 2. The user has at least moderator roles in database.
pub struct ModeratorExtractor(pub AuthorizationToken);

#[async_trait]
impl FromRequestParts<AppState> for ModeratorExtractor {
    type Rejection = ApiGenericResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jwt = JwtUserExtractor::from_request_parts(parts, state).await?.0;
        if jwt.user_profile < Profile::Moderator {
            Err(ApiGenericResponse::UnsifficentPriviledges)
        } else {
            Ok(ModeratorExtractor(jwt))
        }
    }
}

/// Extractor used to check that :
///
/// 1. The user is authenticated.
/// 2. The user has at least admin roles in database.
pub struct AdminExtractor(pub AuthorizationToken);

#[async_trait]
impl FromRequestParts<AppState> for AdminExtractor {
    type Rejection = ApiGenericResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jwt = JwtUserExtractor::from_request_parts(parts, state).await?.0;
        if jwt.user_profile < Profile::Admin {
            Err(ApiGenericResponse::UnsifficentPriviledges)
        } else {
            Ok(AdminExtractor(jwt))
        }
    }
}
