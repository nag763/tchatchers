#[cfg(feature = "back")]
use axum::{http::StatusCode, response::IntoResponse, response::Response};

use crate::locale::Locale;

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
            | ApiResponseKind::MessageDoesNotExist => StatusCode::BAD_REQUEST,
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
    pub trace: uuid::Uuid
}

impl ApiResponse {
    pub fn new(kind: ApiResponseKind, label: &str) -> ApiResponse {
        Self {
            label: label.into(),
            text: Locale::get_default_translation(label),
            kind,
            trace: uuid::Uuid::new_v4()
        }
    }
}

impl TryFrom<String> for ApiResponse {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
    }
}

#[cfg(feature = "back")]
impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let payload = serde_json::to_string(&self).unwrap();
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
            ApiGenericResponse::DbError => {
                ApiResponse::new(ApiResponseKind::DbError, "internal_error")
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
        }
    }
}

#[cfg(feature = "back")]
impl From<sqlx::error::Error> for ApiGenericResponse {
    fn from(_value: sqlx::error::Error) -> Self {
        Self::DbError
    }
}

#[cfg(feature = "back")]
impl IntoResponse for ApiGenericResponse {
    fn into_response(self) -> Response {
        let api_response: ApiResponse = self.into();
        api_response.into_response()
    }
}
