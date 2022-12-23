use std::sync::Arc;

use axum::{
    async_trait,
    body::HttpBody,
    extract::{rejection::JsonRejection, FromRequest},
    http::{Request, StatusCode},
    response::IntoResponse,
    BoxError, Json as JsonAxum,
};
use serde::de::DeserializeOwned;
use tchatchers_core::validation_error_message::ValidationErrorMessage;
use validator::{Validate, ValidationErrors};

use crate::AppState;

pub enum JsonValidatorRejection {
    JsonAxumRejection(JsonRejection),
    ValidationRejection(ValidationErrors),
}

impl IntoResponse for JsonValidatorRejection {
    fn into_response(self) -> axum::response::Response {
        match self {
            JsonValidatorRejection::JsonAxumRejection(rej) => rej.into_response(),
            JsonValidatorRejection::ValidationRejection(errors) => {
                let validation_error_message: ValidationErrorMessage =
                    ValidationErrorMessage::from(errors);
                let error_message = validation_error_message.to_string();
                (StatusCode::BAD_REQUEST, error_message).into_response()
            }
        }
    }
}

pub struct Json<T>(pub T)
where
    T: Validate;

#[async_trait]
impl<B, T> FromRequest<Arc<AppState>, B> for Json<T>
where
    B: 'static + Send + HttpBody,
    B::Data: Send,
    B::Error: Into<BoxError>,
    T: Validate + Sized + DeserializeOwned,
{
    type Rejection = JsonValidatorRejection;

    async fn from_request(req: Request<B>, state: &Arc<AppState>) -> Result<Self, Self::Rejection> {
        match JsonAxum::<T>::from_request(req, state).await {
            Ok(json_value) => {
                if let Err(e) = json_value.validate() {
                    return Err(JsonValidatorRejection::ValidationRejection(e));
                }
                return Ok(Json(json_value.0));
            }
            Err(e) => return Err(JsonValidatorRejection::JsonAxumRejection(e)),
        };
    }
}
