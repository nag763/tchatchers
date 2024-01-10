use axum::{extract::State, http::StatusCode, response::IntoResponse};
use tchatchers_core::{
    api_response::ApiGenericResponse, authorization_status::AuthorizationStatus,
    functional_token::FunctionalToken, serializable_token::SerializableToken, user::User,
};

use crate::{extractor::Postcard, AppState};

pub async fn verify(
    State(state): State<AppState>,
    Postcard(token): Postcard<String>,
) -> Result<impl IntoResponse, ApiGenericResponse> {
    let verification_token: FunctionalToken = FunctionalToken::decode(&token, &state.jwt_secret)?;
    let mut token_pool = state.token_pool.get().await?;
    if verification_token.is_expired() || !verification_token.is_latest(&mut token_pool).await? {
        return Err(ApiGenericResponse::NotValidAnymore);
    }
    match verification_token.token_type {
        tchatchers_core::functional_token::FunctionalTokenType::ValidatingMail => {
            User::update_activation_status(
                verification_token.user_id,
                AuthorizationStatus::Verified,
                &state.pg_pool,
            )
            .await?
        }
    };
    verification_token.consume(&mut token_pool).await?;
    Ok(StatusCode::OK)
}
