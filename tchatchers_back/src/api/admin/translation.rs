use std::{fmt::Display, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tchatchers_core::{manager::ManagerError, translation::TranslationManager};

use crate::{extractor::AdminExtractor, AppState};

pub async fn reload_translations(
    AdminExtractor(_): AdminExtractor,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let mut translation_manager = state.translation_manager.lock().await;
    *translation_manager = TranslationManager::init(&state.pg_pool).await;
    (StatusCode::OK, "Translations reloaded")
}

pub async fn get_all_translations(
    AdminExtractor(_): AdminExtractor,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ManagerError<impl Display>> {
    let translation_manager = state.translation_manager.lock().await;
    let translations = translation_manager.get_all_translations()?;
    Ok(Json(translations))
}

pub async fn get_translations_for_locale(
    Path(locale_id): Path<i32>,
    AdminExtractor(_): AdminExtractor,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ManagerError<impl Display>> {
    let translation_manager = state.translation_manager.lock().await;
    let translation = translation_manager.get_translations_for_locale(locale_id)?;
    Ok(Json(translation))
}
