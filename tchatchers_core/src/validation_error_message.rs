//! Module used to store the common logic for the validation of the user's field
//! and the subsequent error messages displayed.

use std::fmt::Display;

#[cfg(feature = "back")]
use axum::{http::StatusCode, response::IntoResponse};
use validator::ValidationErrors;

impl From<ValidationErrors> for ValidationErrorMessage {
    fn from(errors: ValidationErrors) -> Self {
        let field_errors = errors.field_errors();
        let first_field_in_error = field_errors.iter().next().unwrap();
        let first_error_code = &first_field_in_error.1.first().unwrap().code;
        Self {
            field: first_field_in_error.0.to_string(),
            code: first_error_code.to_string(),
        }
    }
}

/// The error message struct.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValidationErrorMessage {
    /// The concerned error field.
    pub field: String,
    /// The code error on the field.
    pub code: String,
}

impl Display for ValidationErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.code.as_str() {
            "length" => write!(
                f,
                "The {} doesn't respect the length constraints",
                self.field
            ),
            "limited_chars" => write!(f, "The {} doesn't respect the scope of chars allowed.\nOnly letters, numbers, dashes and underscores are allowed.", self.field),
            "security_constraints_not_matched" => write!(f, "The {} doesn't match the security constraints.\nIt is required to have at least one uppercase character, one lowercase character and one number in the {}.", self.field, self.field),
            _ => write!(f, "An error happened druing the validation of the form"),
        }
    }
}

#[cfg(feature = "back")]
impl IntoResponse for ValidationErrorMessage {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}
