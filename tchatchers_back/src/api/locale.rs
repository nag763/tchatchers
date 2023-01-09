use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use tchatchers_core::{locale::Locale, manager::ManagerError};

use crate::{extractor::AdminExtractor, AppState};

pub async fn get_locales(
    AdminExtractor(_): AdminExtractor,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ManagerError<i32>> {
    let locales = state.locale_manager.get_all()?;
    Ok(Json(locales))
}

pub async fn get_locale_id(
    Path(locale_id): Path<i32>,
    AdminExtractor(_): AdminExtractor,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Locale>, ManagerError<i32>> {
    let locales = state.locale_manager.get(locale_id)?;
    Ok(Json(locales))
}
