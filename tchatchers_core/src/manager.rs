use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, derive_more::Display)]
pub enum ManagerError<T: Display> {
    #[display(fmt = "The manager hasn't been init.")]
    NotInit,
    #[display(fmt = "{} hasn't been found.", "_0")]
    NotBound(T),
}

impl<T: Display> axum::response::IntoResponse for ManagerError<T> {
    fn into_response(self) -> axum::response::Response {
        let error_code = match &self {
            Self::NotInit => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotBound(_managed) => axum::http::StatusCode::BAD_REQUEST,
        };
        (error_code, self.to_string()).into_response()
    }
}
