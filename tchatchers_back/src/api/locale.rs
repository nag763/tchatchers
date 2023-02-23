// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! This crate contains some useful tools to get the locales stored in database.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use tchatchers_core::{locale::Locale, manager::ManagerError};

use crate::{extractor::JwtUserExtractor, AppState};

/// Returns the list of locales stored in the database.
pub async fn get_locales(
    JwtUserExtractor(_): JwtUserExtractor,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ManagerError<i32>> {
    let locales = state.locale_manager.get_all()?;
    Ok(Json(locales))
}

/// Returns the locale given as id in the path, returns 404 if not found.
///
/// # Arguments
///
/// - locale_id : The inbase id of the locale in the database.
pub async fn get_locale_id(
    Path(locale_id): Path<i32>,
    JwtUserExtractor(_): JwtUserExtractor,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Locale>, ManagerError<i32>> {
    let locales = state.locale_manager.get(locale_id)?;
    Ok(Json(locales))
}
