use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tchatchers_core::app_context::AppContext;

use crate::{extractor::JwtUserExtractor, AppState};

pub async fn app_context(
    JwtUserExtractor(jwt): JwtUserExtractor,
    state: State<Arc<AppState>>,
) -> impl IntoResponse {
    match state
        .translation_manager
        .get_translations_for_locale(jwt.user.locale_id)
        .await
    {
        Ok(translation) => Ok((
            StatusCode::OK,
            Json(AppContext {
                user: jwt.user,
                translation,
            }),
        )),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
