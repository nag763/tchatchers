// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! This module contains the route allowing the user to access the application context.
//!
//! The context is useful since it permits the front end to have an understandable and parametized view
//! given the user that uses the application.

use std::rc::Rc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tchatchers_core::{
    app_context::UserContext, navlink::Navlink, translation::Translation, user::User,
};

use crate::{extractor::JwtUserExtractor, AppState};

/// Route to get the application context.
///
/// This requires the user to be authenticated. Once the user is authenticated,
/// will be returned a lot of data useful when browsing the application, allowing
/// an asynchronous browsing.
pub async fn user_context(
    JwtUserExtractor(jwt): JwtUserExtractor,
    state: State<AppState>,
) -> Result<impl IntoResponse, Response> {
    let Some(user) = User::find_by_id(jwt.user_id, &state.pg_pool).await else  {
        return Err((StatusCode::FORBIDDEN, "User doesn't exist anymore, please log out.").into_response());
    };
    if !user.is_authorized {
        return Err((
            StatusCode::FORBIDDEN,
            "This user's access has been deactivated, please log out.",
        )
            .into_response());
    }
    let translation: Translation = state
        .translation_manager
        .lock()
        .await
        .get_translations_for_locale(user.locale_id)
        .map_err(|e| e.into_response())?;
    let navlink: Vec<Navlink> = state
        .navlink_manager
        .lock()
        .await
        .get_navlink_for_profile(user.profile)
        .map_err(|e| e.into_response())?;
    let available_locale = state
        .locale_manager
        .get_all()
        .map_err(|e| e.into_response())?;
    Ok(Json(UserContext {
        user: user.into(),
        navlink,
        translation: Rc::new(translation),
        available_locale,
    }))
}
