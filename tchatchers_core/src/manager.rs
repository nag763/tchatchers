// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! Module containing the common errors that can be returned from the managers.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// The error returned while using a manager.
#[derive(Clone, Debug, Serialize, Deserialize, derive_more::Display)]
pub enum ManagerError<T: Display> {
    /// If the manager is not initialized, this will be returned.
    #[display(fmt = "The manager hasn't been init.")]
    NotInit,
    /// If a specific indentifier isn't found, this will be returned.
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
