use std::fmt::Display;

use validator::ValidationErrors;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValidationErrorMessage {
    field: String,
    code: String,
}

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

impl Display for ValidationErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.code.as_str() {
            "length" => write!(
                f,
                "The {} doesn't respect the length constraints",
                self.field
            ),
            _ => write!(f, "An error happened druing the validation of the form"),
        }
    }
}
