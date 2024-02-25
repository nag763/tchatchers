#[cfg(feature = "back")]
use axum::{http::StatusCode, response::IntoResponse, response::Response};
use validator::ValidationErrors;

use crate::{locale::Locale, validation_error_message::ValidationErrorMessage};

#[derive(serde::Deserialize, serde::Serialize)]
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
    IoError,
    RedisError,
    TooManyRequests,
    MultipartError,
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
            | ApiResponseKind::SerializationError
            | ApiResponseKind::MultipartError => StatusCode::BAD_REQUEST,
            ApiResponseKind::AuthenticationRequired | ApiResponseKind::AuthenticationExpired => {
                StatusCode::UNAUTHORIZED
            }
            ApiResponseKind::UnsifficentPriviledges | ApiResponseKind::AccessRevoked => {
                StatusCode::FORBIDDEN
            }
            ApiResponseKind::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            ApiResponseKind::DbError | ApiResponseKind::IoError | ApiResponseKind::RedisError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
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
}

impl ApiResponse {
    pub fn new(kind: ApiResponseKind, label: &str) -> ApiResponse {
        Self {
            label: label.into(),
            text: Locale::get_default_translation(label),
            kind,
            trace: uuid::Uuid::new_v4(),
            errors: vec![],
        }
    }

    pub fn errors(kind: ApiResponseKind, label: &str, errors: Vec<String>) -> ApiResponse {
        Self {
            label: label.into(),
            text: Locale::get_default_translation(label),
            kind,
            trace: uuid::Uuid::new_v4(),
            errors,
        }
    }
}

#[cfg(feature = "back")]
impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let payload = postcard::to_stdvec(&self).unwrap();
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
    RedisError(String),
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
    IoError(String),
    TooManyRequests,
    MultipartError(String),
}

impl From<ApiGenericResponse> for ApiResponse {
    fn from(value: ApiGenericResponse) -> Self {
        match value {
            ApiGenericResponse::AuthenticationRequired => ApiResponse::new(
                ApiResponseKind::AuthenticationRequired,
                "route_requires_authentication",
            ),
            ApiGenericResponse::AuthenticationExpired => ApiResponse::new(
                ApiResponseKind::AuthenticationExpired,
                "authentication_expired",
            ),
            ApiGenericResponse::UnsifficentPriviledges => ApiResponse::new(
                ApiResponseKind::UnsifficentPriviledges,
                "unsufficient_priviledges",
            ),
            ApiGenericResponse::SimilarLoginExists => {
                ApiResponse::new(ApiResponseKind::SimilarLoginExists, "similar_login_exists")
            }
            ApiGenericResponse::DbError(errors) => {
                ApiResponse::errors(ApiResponseKind::DbError, "internal_error", vec![errors])
            }
            ApiGenericResponse::UserCreated => {
                ApiResponse::new(ApiResponseKind::UserCreated, "user_created")
            }
            ApiGenericResponse::BadCredentials => {
                ApiResponse::new(ApiResponseKind::BadCredentials, "bad_credentials")
            }
            ApiGenericResponse::AccessRevoked => {
                ApiResponse::new(ApiResponseKind::AccessRevoked, "access_revoked")
            }
            ApiGenericResponse::AccountNotFound => {
                ApiResponse::new(ApiResponseKind::AccountNotFound, "account_not_found")
            }
            ApiGenericResponse::UserAlreadyReported => ApiResponse::new(
                ApiResponseKind::UserAlreadyReported,
                "user_already_reported",
            ),
            ApiGenericResponse::UserNotFound => {
                ApiResponse::new(ApiResponseKind::UserNotFound, "user_not_found")
            }
            ApiGenericResponse::MessageDoesNotExist => ApiResponse::new(
                ApiResponseKind::MessageDoesNotExist,
                "message_does_not_exist",
            ),
            ApiGenericResponse::MessageAlreadyReported => ApiResponse::new(
                ApiResponseKind::MessageAlreadyReported,
                "message_already_reported",
            ),
            ApiGenericResponse::MessageReported => {
                ApiResponse::new(ApiResponseKind::MessageReported, "message_reported")
            }
            ApiGenericResponse::MessageDeleted => {
                ApiResponse::new(ApiResponseKind::MessageDeleted, "message_deleted")
            }
            ApiGenericResponse::UserReported => {
                ApiResponse::new(ApiResponseKind::UserReported, "user_reported")
            }
            ApiGenericResponse::RevokedUser => {
                ApiResponse::new(ApiResponseKind::RevokedUser, "revoked_user")
            }
            ApiGenericResponse::ByteRejection(errors) => ApiResponse::errors(
                ApiResponseKind::ByteRejection,
                "expected_byte",
                vec![errors],
            ),
            ApiGenericResponse::SerializationError(errors) => ApiResponse::errors(
                ApiResponseKind::SerializationError,
                "serialization_error",
                vec![errors],
            ),
            ApiGenericResponse::ValidationError(errors) => {
                ApiResponse::errors(ApiResponseKind::ValidationError, "validation_error", errors)
            }
            ApiGenericResponse::ContentTypeError => ApiResponse::new(
                ApiResponseKind::ContentTypeError,
                "content_type_missing_or_not_accepted",
            ),
            ApiGenericResponse::IoError(e) => {
                ApiResponse::errors(ApiResponseKind::IoError, "io_error", vec![e.to_string()])
            }
            ApiGenericResponse::RedisError(e) => ApiResponse::errors(
                ApiResponseKind::RedisError,
                "internal_error",
                vec![e.to_string()],
            ),
            ApiGenericResponse::TooManyRequests => ApiResponse::errors(
                ApiResponseKind::TooManyRequests,
                "max_conns_reached",
                vec!["Too many requests received from the client".to_string()],
            ),
            ApiGenericResponse::MultipartError(e) => {
                ApiResponse::errors(ApiResponseKind::MultipartError, "multipart_error", vec![e])
            }
        }
    }
}

#[cfg(feature = "back")]
impl From<sqlx::error::Error> for ApiGenericResponse {
    fn from(value: sqlx::error::Error) -> Self {
        Self::DbError(value.to_string())
    }
}

#[cfg(feature = "back")]
impl IntoResponse for ApiGenericResponse {
    fn into_response(self) -> Response {
        let api_response: ApiResponse = self.into();
        api_response.into_response()
    }
}

#[cfg(feature = "back")]
impl From<axum::extract::rejection::BytesRejection> for ApiGenericResponse {
    fn from(value: axum::extract::rejection::BytesRejection) -> Self {
        Self::ByteRejection(value.to_string())
    }
}

#[cfg(feature = "back")]
impl From<bb8_redis::bb8::RunError<redis::RedisError>> for ApiGenericResponse {
    fn from(value: bb8_redis::bb8::RunError<redis::RedisError>) -> Self {
        Self::DbError(value.to_string())
    }
}

#[cfg(feature = "back")]
impl From<axum::extract::multipart::MultipartError> for ApiGenericResponse {
    fn from(value: axum::extract::multipart::MultipartError) -> Self {
        Self::MultipartError(value.to_string())
    }
}

impl From<std::io::Error> for ApiGenericResponse {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

#[cfg(feature = "back")]
impl From<jsonwebtoken::errors::Error> for ApiGenericResponse {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::SerializationError(value.to_string())
    }
}

#[cfg(feature = "back")]
impl From<redis::RedisError> for ApiGenericResponse {
    fn from(value: redis::RedisError) -> Self {
        Self::RedisError(value.to_string())
    }
}

impl From<postcard::Error> for ApiGenericResponse {
    fn from(value: postcard::Error) -> Self {
        Self::SerializationError(value.to_string())
    }
}

impl From<serde_json::Error> for ApiGenericResponse {
    fn from(value: serde_json::Error) -> Self {
        Self::SerializationError(value.to_string())
    }
}

impl From<ValidationErrors> for ApiGenericResponse {
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
        ApiGenericResponse::ValidationError(errors)
    }
}
