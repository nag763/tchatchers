// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! Defines the extractors used by the different webservices.

use crate::AppState;
use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::{FromRequest, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::{header, request::Parts, HeaderValue, Request},
    response::IntoResponse,
    BoxError, TypedHeader,
};
use serde::{de::DeserializeOwned, Serialize};
use tchatchers_core::{
    api_response::{ApiGenericResponse, AcceptedContentType, ApiResponse}, authorization_token::AuthorizationToken, profile::Profile,
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

    axum::Json

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

#[async_trait]
impl FromRequestParts<AppState> for AcceptedContentType {
    type Rejection = ApiGenericResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let content_type : AcceptedContentType = (&parts.headers).try_into()?;
        Ok(content_type)

    }
}

/// A validated JSON input.
///
/// Mainly used to validate the data before processing it server side.
pub struct Payload<T>(pub AcceptedContentType, pub T);

#[async_trait]
impl<B, T> FromRequest<AppState, B> for Payload<T>
where
    B: 'static + Send + HttpBody,
    B::Data: Send,
    B::Error: Into<BoxError>,
    T: Sized + DeserializeOwned,
{
    type Rejection = ApiResponse;

    async fn from_request(req: Request<B>, state: &AppState) -> Result<Self, Self::Rejection> {
        let content_type = req.headers().try_into()?;

        let b = Bytes::from_request(req, state).await?;
        let entity = match content_type {
            AcceptedContentType::Json =>  serde_json::from_slice(&b).unwrap(),
            AcceptedContentType::Postcard => postcard::from_bytes(&b)?,
            _ => return Err(ApiGenericResponse::ContentTypeError)
        };
        Ok(Payload(content_type,entity))
    }
}

pub struct ValidPayload<T>(pub T)
where
    T: Validate;

#[async_trait]
impl<B, T> FromRequest<AppState, B> for ValidPayload<T>
where
    B: 'static + Send + HttpBody<Data = Bytes> + std::marker::Unpin,
    B::Data: Send,
    B::Error: Into<BoxError>,
    T: Sized + DeserializeOwned + Validate,
{
    type Rejection = ApiGenericResponse;

    async fn from_request(req: Request<B>, state: &AppState) -> Result<Self, Self::Rejection> {
        let entity: Payload<T> = Payload::from_request(req, state).await?;
        entity.1.validate()?;
        Ok(ValidPayload(entity.1))
    }
}

impl<T: Serialize> IntoResponse for Payload<T> {
    fn into_response(self) -> axum::response::Response {
        let body = match self.0 {
            AcceptedContentType::Json => 
                match postcard::to_stdvec(&self.1) {
                    Ok(v) => v,
                    Err(e) => return ApiGenericResponse::SerializationError(e.to_string()).into_response(),
                }
            ,
            AcceptedContentType::Postcard => match serde_json::to_vec(&self.1) {
                Ok(v) => v,
                Err(e) => return ApiGenericResponse::SerializationError(e.to_string()).into_response(),
            }
        };
             (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(POSTCARD_CONTENT_TYPE),
                )],
                body,
            )
                .into_response()
    }
}
