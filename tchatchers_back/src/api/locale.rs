use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use tchatchers_core::locale::Locale;

use crate::{extractor::AdminExtractor, AppState};

pub async fn get_locales(
    AdminExtractor(_): AdminExtractor,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    Json(Locale::get_all(&state.pg_pool).await)
}

pub async fn get_locale_id(
    Path(locale_id): Path<i32>,
    AdminExtractor(_): AdminExtractor,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    Json(Locale::get_locale_id(locale_id, &state.pg_pool).await)
}
