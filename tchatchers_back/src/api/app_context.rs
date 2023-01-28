// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! This module contains the route allowing the user to access the application context.
//! 
//! The context is useful since it permits the front end to have an understandable and parametized view
//! given the user that uses the application.

use std::{rc::Rc, sync::Arc};

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use tchatchers_core::{app_context::AppContext, navlink::Navlink, translation::Translation};

use crate::{extractor::JwtUserExtractor, AppState};


/// Route to get the application context.
/// 
/// This requires the user to be authenticated. Once the user is authenticated, 
/// will be returned a lot of data useful when browsing the application, allowing
/// an asynchronous browsing.
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
    let available_locale = state
        .locale_manager
        .get_all()
        .map_err(|e| e.into_response())?;
    Ok(Json(AppContext {
        user: jwt.user,
        navlink,
        translation: Rc::new(translation),
        available_locale,
    }))
}
