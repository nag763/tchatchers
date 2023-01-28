// Copyright eⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! Administrative translations tools.
//! 
//! This module contains the routes allowing to perform administrative actions on the translations stored in database, and help
//! the users facing issues to find what is happening.

use std::{fmt::Display, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tchatchers_core::{manager::ManagerError, translation::TranslationManager};

use crate::{extractor::AdminExtractor, AppState};


/// Reload the translations from the database.
/// 
/// This allows a refresh of the cache manager, useful when a translation has been inserted in database
/// and we want it to be displayed on the next connections.
pub async fn reload_translations(
    AdminExtractor(_): AdminExtractor,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let mut translation_manager = state.translation_manager.lock().await;
    *translation_manager = TranslationManager::init(&state.pg_pool).await;
    (StatusCode::OK, "Translations reloaded")
}


/// Returns all the translations available in the cache manager.
/// 
/// The translations will be returned a JSON map with the keys being the locale id and the values being
/// the translations. 
pub async fn get_all_translations(
    AdminExtractor(_): AdminExtractor,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ManagerError<impl Display>> {
    let translation_manager = state.translation_manager.lock().await;
    let translations = translation_manager.get_all_translations()?;
    Ok(Json(translations))
}

/// Returns all the translations for a given locale.
/// 
/// # Arguments
/// 
/// - locale_id : The locale the translations need to be fetched from.
pub async fn get_translations_for_locale(
    Path(locale_id): Path<i32>,
    AdminExtractor(_): AdminExtractor,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ManagerError<impl Display>> {
    let translation_manager = state.translation_manager.lock().await;
    let translation = translation_manager.get_translations_for_locale(locale_id)?;
    Ok(Json(translation))
}
