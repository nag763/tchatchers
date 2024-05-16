#[cfg(feature = "back")]
use axum::{http::StatusCode, response::IntoResponse, response::Response};
use validator::ValidationErrors;

use crate::{locale::Locale, validation_error_message::ValidationErrorMessage};

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
        let payload = bincode::serialize(&self).unwrap();
        let code: StatusCode = self.kind.into();
        (code, payload).into_response()
    }
}

#[derive(serde::Deserialize, serde::Serialize, IntoApiResponse, ErrorWrapper)]
pub enum ApiGenericResponse {
    #[response(status=UNAUTHORIZED, simple("route_requires_authentication"))]
    AuthenticationRequired,
    #[response(status=UNAUTHORIZED, simple)]
    AuthenticationExpired,
    #[response(status=FORBIDDEN, simple)]
    UnsifficentPriviledges,
    #[response(status=CONFLICT, simple("similar_login_exists"))]
    SimilarLoginExists,
    #[response(status=INTERNAL_SERVER_ERROR, error("db_error"))]
    #[cfg_attr(feature = "back", from_err(sqlx::error::Error, redis::RedisError))]
    DbError(String),
    #[response(status=INTERNAL_SERVER_ERROR, error("redis_error"))]
    RedisError(String),
    #[response(status=CREATED, simple("user_created"))]
    UserCreated,
    #[response(status=BAD_REQUEST, simple("bad_credentials"))]
    BadCredentials,
    #[response(status=FORBIDDEN, simple("access_revoked"))]
    AccessRevoked,
    #[response(status=NOT_FOUND, simple("account_not_found"))]
    AccountNotFound,
    #[response(status=OK, simple("user_already_reported"))]
    UserAlreadyReported,
    #[response(status=NOT_FOUND, simple("user_not_found"))]
    UserNotFound,
    #[response(status=NOT_FOUND, simple("message_does_not_exist"))]
    MessageDoesNotExist,
    #[response(status=OK, simple("message_already_reported"))]
    MessageAlreadyReported,
    #[response(status=OK, simple("message_reported"))]
    MessageReported,
    #[response(status=OK, simple("message_deleted"))]
    MessageDeleted,
    #[response(status=OK, simple("user_reported"))]
    UserReported,
    #[response(status=OK, simple("revoked_user"))]
    RevokedUser,
    #[response(status=BAD_REQUEST, error("expected_byte"))]
    #[cfg_attr(feature = "back", from_err(axum::extract::rejection::BytesRejection))]
    ByteRejection(String),
    #[response(status=BAD_REQUEST, error("serialization_error"))]
    #[from_err(bincode::Error)]
    #[cfg_attr(feature = "back", from_err(jsonwebtoken::errors::Error, serde_json::Error))]
    SerializationError(String),
    #[response(status=BAD_REQUEST, errors("validation_error"))]
    ValidationError(Vec<String>),
    #[response(status=BAD_REQUEST, simple("content_type_error"))]
    ContentTypeError,
    #[response(status=INTERNAL_SERVER_ERROR, error("io_error"))]
    #[from_err(std::io::Error)]
    IoError(String),
    #[response(status=TOO_MANY_REQUESTS, simple("max_conns_reached"))]
    TooManyRequests,
    #[response(status=BAD_REQUEST, error("multipart_error"))]
    #[cfg_attr(feature = "back", from_err(axum::extract::multipart::MultipartError))]
    MultipartError(String),
}

#[cfg(feature = "back")]
impl IntoResponse for ApiGenericResponse {
    fn into_response(self) -> Response {
        let api_response: ApiResponse = self.into();
        api_response.into_response()
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
