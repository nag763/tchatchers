#[cfg(feature = "back")]
use axum::{http::StatusCode, response::IntoResponse, response::Response};
use derive_more::Display;
use validator::ValidationErrors;

use crate::{locale::Locale, validation_error_message::ValidationErrorMessage};

#[derive(serde::Serialize, serde::Deserialize)]
pub enum AcceptedContentType {
    Json,
    Postcard
}

impl TryFrom<&str> for AcceptedContentType {
    type Error = ApiGenericResponse;

    fn try_from(value: &str) -> Result<Self, ApiGenericResponse> {
        match value {
            "json" => Ok(Self::Json),
            "postcard" => Ok(Self::Postcard),
            _ => return Err(ApiGenericResponse::ContentTypeError) 
        }
    }
}

#[cfg(feature = "back")]
impl TryFrom<&axum::http::HeaderMap> for AcceptedContentType {
    type Error = ApiGenericResponse;

    fn try_from(value: &axum::http::HeaderMap) -> Result<Self, Self::Error> {

        let Some(content_type) = value.get(axum::http::header::CONTENT_TYPE) else {
            return Err(ApiGenericResponse::ContentTypeError);
        };
        
        let Ok(content_type) = content_type.to_str() else {
            return Err(ApiGenericResponse::ContentTypeError);
        };
    
        let Ok(mime) =  content_type.parse::<mime::Mime>() else {
            return Err(ApiGenericResponse::ContentTypeError);
        };
    
        if mime.type_() == "application" {
            let content_type : AcceptedContentType = mime.subtype().as_str().try_into()?;
            Ok(content_type)
        } else {
            return Err(ApiGenericResponse::ContentTypeError);
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Display)]
pub enum ApiResponseKind {
    AuthenticationRequired,
    AuthenticationExpired,
    UnsifficentPriviledges,
    SimilarLoginExists,
    DbError,
    UserCreated,
    BadCredentials,
    AccessRevoked,
    AccountNotFound,
    UserAlreadyReported,
    UserNotFound,
    MessageDoesNotExist,
    MessageAlreadyReported,
    MessageReported,
    MessageDeleted,
    UserReported,
    RevokedUser,
    ByteRejection,
    SerializationError,
    ValidationError,
    ContentTypeError,
}

#[cfg(feature = "back")]
impl From<ApiResponseKind> for StatusCode {
    fn from(value: ApiResponseKind) -> StatusCode {
        match value {
            ApiResponseKind::SimilarLoginExists => StatusCode::CONFLICT,
            ApiResponseKind::UserCreated => StatusCode::CREATED,
            ApiResponseKind::MessageReported
            | ApiResponseKind::MessageDeleted
            | ApiResponseKind::UserReported
            | ApiResponseKind::RevokedUser => StatusCode::OK,
            ApiResponseKind::MessageAlreadyReported
            | ApiResponseKind::UserNotFound
            | ApiResponseKind::UserAlreadyReported
            | ApiResponseKind::AccountNotFound
            | ApiResponseKind::BadCredentials
            | ApiResponseKind::MessageDoesNotExist
            | ApiResponseKind::ByteRejection
            | ApiResponseKind::ValidationError
            | ApiResponseKind::ContentTypeError
            | ApiResponseKind::SerializationError => StatusCode::BAD_REQUEST,
            ApiResponseKind::AuthenticationRequired | ApiResponseKind::AuthenticationExpired => {
                StatusCode::UNAUTHORIZED
            }
            ApiResponseKind::UnsifficentPriviledges | ApiResponseKind::AccessRevoked => {
                StatusCode::FORBIDDEN
            }
            ApiResponseKind::DbError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ApiResponse {
    pub kind: ApiResponseKind,
    pub label: String,
    pub text: Option<String>,
    pub trace: uuid::Uuid,
    pub errors: Vec<String>,
    pub content_type: Option<AcceptedContentType>
}

impl ApiResponse {
    pub fn new(kind: ApiResponseKind, content_type: Option<AcceptedContentType>) -> ApiResponse {
        let label = kind.to_string();
        Self {
            text: Locale::get_default_translation(&label),
            label,
            kind,
            trace: uuid::Uuid::new_v4(),
            errors: vec![],
            content_type: content_type
        }
    }

    pub fn errors(kind: ApiResponseKind, errors: Vec<String>, content_type: Option<AcceptedContentType>) -> ApiResponse {
        let label = kind.to_string();
        Self {
            text: Locale::get_default_translation(&label),
            label,
            kind,
            trace: uuid::Uuid::new_v4(),
            errors,
            content_type: content_type
        }
    }
}

#[cfg(feature = "back")]
impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let payload = match self.content_type.as_ref().unwrap_or(&AcceptedContentType::Json) {
            AcceptedContentType::Json => serde_json::to_vec(&self).unwrap(),
            AcceptedContentType::Postcard => postcard::to_stdvec(&self).unwrap(),
        }; 
        let code: StatusCode = self.kind.into();
        (code, payload).into_response()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum ApiGenericResponse {
    AuthenticationRequired,
    AuthenticationExpired,
    UnsifficentPriviledges,
    SimilarLoginExists,
    DbError(String),
    UserCreated,
    BadCredentials,
    AccessRevoked,
    AccountNotFound,
    UserAlreadyReported,
    UserNotFound,
    MessageDoesNotExist,
    MessageAlreadyReported,
    MessageReported,
    MessageDeleted,
    UserReported,
    RevokedUser,
    ByteRejection(String),
    SerializationError(String),
    ValidationError(Vec<String>),
    ContentTypeError,
}

// impl From<ApiGenericResponse> for ApiResponse {
//     fn from(value: ApiGenericResponse) -> Self {
//         match value {
//             ApiGenericResponse::AuthenticationRequired => ApiResponse::new(
//                 ApiResponseKind::AuthenticationRequired,
//                 "route_requires_authentication",
//             ),
//             ApiGenericResponse::AuthenticationExpired => ApiResponse::new(
//                 ApiResponseKind::AuthenticationExpired,
//                 "authentication_expired",
//             ),
//             ApiGenericResponse::UnsifficentPriviledges => ApiResponse::new(
//                 ApiResponseKind::UnsifficentPriviledges,
//                 "unsufficient_priviledges",
//             ),
//             ApiGenericResponse::SimilarLoginExists => {
//                 ApiResponse::new(ApiResponseKind::SimilarLoginExists, "similar_login_exists")
//             }
//             ApiGenericResponse::DbError(errors) => {
//                 ApiResponse::errors(ApiResponseKind::DbError, "internal_error", vec![errors])
//             }
//             ApiGenericResponse::UserCreated => {
//                 ApiResponse::new(ApiResponseKind::UserCreated, "user_created")
//             }
//             ApiGenericResponse::BadCredentials => {
//                 ApiResponse::new(ApiResponseKind::BadCredentials, "bad_credentials")
//             }
//             ApiGenericResponse::AccessRevoked => {
//                 ApiResponse::new(ApiResponseKind::AccessRevoked, "access_revoked")
//             }
//             ApiGenericResponse::AccountNotFound => {
//                 ApiResponse::new(ApiResponseKind::AccountNotFound, "account_not_found")
//             }
//             ApiGenericResponse::UserAlreadyReported => ApiResponse::new(
//                 ApiResponseKind::UserAlreadyReported,
//                 "user_already_reported",
//             ),
//             ApiGenericResponse::UserNotFound => {
//                 ApiResponse::new(ApiResponseKind::UserNotFound, "user_not_found")
//             }
//             ApiGenericResponse::MessageDoesNotExist => ApiResponse::new(
//                 ApiResponseKind::MessageDoesNotExist,
//                 "message_does_not_exist",
//             ),
//             ApiGenericResponse::MessageAlreadyReported => ApiResponse::new(
//                 ApiResponseKind::MessageAlreadyReported,
//                 "message_already_reported",
//             ),
//             ApiGenericResponse::MessageReported => {
//                 ApiResponse::new(ApiResponseKind::MessageReported, "message_reported")
//             }
//             ApiGenericResponse::MessageDeleted => {
//                 ApiResponse::new(ApiResponseKind::MessageDeleted, "message_deleted")
//             }
//             ApiGenericResponse::UserReported => {
//                 ApiResponse::new(ApiResponseKind::UserReported, "user_reported")
//             }
//             ApiGenericResponse::RevokedUser => {
//                 ApiResponse::new(ApiResponseKind::RevokedUser, "revoked_user")
//             }
//             ApiGenericResponse::ByteRejection(errors) => ApiResponse::errors(
//                 ApiResponseKind::ByteRejection,
//                 "expected_byte",
//                 vec![errors],
//             ),
//             ApiGenericResponse::SerializationError(errors) => ApiResponse::errors(
//                 ApiResponseKind::SerializationError,
//                 "serialization_error",
//                 vec![errors],
//             ),
//             ApiGenericResponse::ValidationError(errors) => {
//                 ApiResponse::errors(ApiResponseKind::ValidationError, "validation_error", errors)
//             }
//             ApiGenericResponse::ContentTypeError => ApiResponse::new(
//                 ApiResponseKind::ContentTypeError,
//                 "content_type_missing_or_not_accepted",
//             ),
//         }
//     }
// }

#[cfg(feature = "back")]
impl From<sqlx::error::Error> for ApiGenericResponse {
    fn from(value: sqlx::error::Error) -> Self {
        Self::DbError(value.to_string())
    }
}

// #[cfg(feature = "back")]
// impl IntoResponse for ApiGenericResponse {
//     fn into_response(self) -> Response {
//         let api_response: ApiResponse = self.into();
//         api_response.into_response()
//     }
// }

#[cfg(feature = "back")]
impl From<axum::extract::rejection::BytesRejection> for ApiResponse {
    fn from(value: axum::extract::rejection::BytesRejection) -> Self {
        ApiResponse::errors(ApiResponseKind::ByteRejection, vec![value.to_string()], None)
    }
}

impl From<postcard::Error> for ApiResponse {
    fn from(value: postcard::Error) -> Self {
        ApiResponse::errors(ApiResponseKind::SerializationError, vec![value.to_string()], None)
    }
}

impl From<serde_json::Error> for ApiResponse {
    fn from(value: serde_json::Error) -> Self {
        ApiResponse::errors(ApiResponseKind::SerializationError, vec![value.to_string()], None)
    }
}

impl From<ValidationErrors> for ApiResponse {
    fn from(value: ValidationErrors) -> Self {
        let mut validation_errors: Vec<ValidationErrorMessage> = vec![];
        for errors in value.field_errors() {
            let field = errors.0;
            for field_errors in errors.1 {
                validation_errors.push(ValidationErrorMessage {
                    field: field.to_string(),
                    code: field_errors.code.to_string(),
                })
            }
        }
        let errors = validation_errors.iter().map(|v| v.to_string()).collect();
        ApiResponse::errors(ApiResponseKind::ValidationError, errors, None)
    }
}
