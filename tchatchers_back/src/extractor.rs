// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! Defines the extractors used by the different webservices.

use crate::AppState;
use axum::{
    async_trait,
    body::{Body, Bytes},
    extract::{FromRequest, FromRequestParts},
    http::{header, request::Parts, HeaderMap, HeaderValue, Request},
    response::IntoResponse,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{de::DeserializeOwned, Serialize};
use tchatchers_core::{
    api_response::ApiGenericResponse, authorization_token::AuthorizationToken, profile::Profile,
    serializable_token::SerializableToken,
};
use validator::Validate;

static BINCODE_CONTENT_TYPE: &str = "application/bincode";

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
        let Ok(TypedHeader(Authorization(jwt))) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await
        else {
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

fn bincode_content_type(headers: &HeaderMap) -> bool {
    let content_type = if let Some(content_type) = headers.get(header::CONTENT_TYPE) {
        content_type
    } else {
        return false;
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return false;
    };

    let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
        mime
    } else {
        return false;
    };

    let is_bincode_content_type = mime.type_() == "application"
        && (mime.subtype() == "bincode" || mime.suffix().map_or(false, |name| name == "bincode"));

    is_bincode_content_type
}

/// A validated JSON input.
///
/// Mainly used to validate the data before processing it server side.
pub struct Bincode<T>(pub T);

#[async_trait]
impl<T> FromRequest<AppState> for Bincode<T>
where
    T: Sized + DeserializeOwned,
{
    type Rejection = ApiGenericResponse;

    async fn from_request(
        req: axum::http::Request<Body>,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        if !bincode_content_type(req.headers()) {
            return Err(ApiGenericResponse::ContentTypeError);
        }
        let b = Bytes::from_request(req, state).await?;
        let entity = bincode::deserialize(&b)?;
        Ok(Bincode(entity))
    }
}

pub struct ValidBincode<T>(pub T)
where
    T: Validate;

#[async_trait]
impl<T> FromRequest<AppState> for ValidBincode<T>
where
    T: Sized + DeserializeOwned + Validate,
{
    type Rejection = ApiGenericResponse;

    async fn from_request(req: Request<Body>, state: &AppState) -> Result<Self, Self::Rejection> {
        let entity: Bincode<T> = Bincode::from_request(req, state).await?;
        entity.0.validate()?;
        Ok(ValidBincode(entity.0))
    }
}

impl<T: Serialize> IntoResponse for Bincode<T> {
    fn into_response(self) -> axum::response::Response {
        match bincode::serialize(&self.0) {
            Ok(v) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(BINCODE_CONTENT_TYPE),
                )],
                v,
            )
                .into_response(),
            Err(e) => ApiGenericResponse::from(e).into_response(),
        }
    }
}
