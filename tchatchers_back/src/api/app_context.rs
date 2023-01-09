use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use tchatchers_core::{
    app_context::AppContext, navlink::Navlink, translation::Translation,
};

use crate::{extractor::JwtUserExtractor, AppState};

pub async fn app_context(
    JwtUserExtractor(jwt): JwtUserExtractor,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, Response> {
    let translation: Translation = state
        .translation_manager
        .lock()
        .await
        .get_translations_for_locale(jwt.user.locale_id)
        .map_err(|e| e.into_response())?;
    let navlink: Vec<Navlink> = state
        .navlink_manager
        .lock()
        .await
        .get_navlink_for_profile(jwt.user.profile)
        .map_err(|e| e.into_response())?;
    let available_locale = state.locale_manager.get_all().map_err(|e| e.into_response())?;
    Ok(Json(AppContext {
        user: jwt.user,
        navlink,
        translation,
        available_locale
    }))
}
