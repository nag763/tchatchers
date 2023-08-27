// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! Defines the extractors used by the different webservices.

use crate::AppState;
use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::{FromRequest, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::{header, request::Parts, HeaderMap, HeaderValue, Request},
    response::IntoResponse,
    BoxError, TypedHeader,
};
use serde::{de::DeserializeOwned, Serialize};
use tchatchers_core::{
    api_response::ApiGenericResponse, authorization_token::AuthorizationToken, profile::Profile,
    serializable_token::SerializableToken,
};
use validator::Validate;

static POSTCARD_CONTENT_TYPE: &str = "application/postcard";

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

fn postcard_content_type(headers: &HeaderMap) -> bool {
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

    let is_postcard_content_type = mime.type_() == "application"
        && (mime.subtype() == "postcard" || mime.suffix().map_or(false, |name| name == "postcard"));

    is_postcard_content_type
}

/// A validated JSON input.
///
/// Mainly used to validate the data before processing it server side.
pub struct Postcard<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<AppState, B> for Postcard<T>
where
    B: 'static + Send + HttpBody,
    B::Data: Send,
    B::Error: Into<BoxError>,
    T: Sized + DeserializeOwned,
{
    type Rejection = ApiGenericResponse;

    async fn from_request(req: Request<B>, state: &AppState) -> Result<Self, Self::Rejection> {
        if !postcard_content_type(req.headers()) {
            return Err(ApiGenericResponse::ContentTypeError);
        }
        let b = Bytes::from_request(req, state).await?;
        let entity = postcard::from_bytes(&b)?;
        Ok(Postcard(entity))
    }
}

pub struct ValidPostcard<T>(pub T)
where
    T: Validate;

#[async_trait]
impl<B, T> FromRequest<AppState, B> for ValidPostcard<T>
where
    B: 'static + Send + HttpBody,
    B::Data: Send,
    B::Error: Into<BoxError>,
    T: Sized + DeserializeOwned + Validate,
{
    type Rejection = ApiGenericResponse;

    async fn from_request(req: Request<B>, state: &AppState) -> Result<Self, Self::Rejection> {
        let entity: Postcard<T> = Postcard::from_request(req, state).await?;
        entity.0.validate()?;
        Ok(ValidPostcard(entity.0))
    }
}

impl<T: Serialize> IntoResponse for Postcard<T> {
    fn into_response(self) -> axum::response::Response {
        match postcard::to_stdvec(&self.0) {
            Ok(v) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(POSTCARD_CONTENT_TYPE),
                )],
                v,
            )
                .into_response(),
            Err(e) => ApiGenericResponse::from(e).into_response(),
        }
    }
}
