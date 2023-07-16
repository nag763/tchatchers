// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! The application context is a set of data used by the front-end application to understand who is the current user, and is provided
//! by the server at the login.

use std::rc::Rc;

use crate::{
    locale::{Locale, Translation},
    navlink::Navlink,
    user::PartialUser,
};

/// The application context.
///
/// Contains several important data that the user needs in order to browse the application.
#[derive(Debug, PartialEq, Eq, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UserContext {
    /// The user currently being logged in.
    pub user: PartialUser,
    /// The translations associated to the user's locale.
    ///
    /// Mainly used for internationalization.
    pub translation: Rc<Translation>,
    /// The list of navigation links he can access to.
    ///
    /// This depends on the user's profile.
    pub navlink: Vec<Navlink>,
    /// The locales available.
    ///
    /// Mainly used to not request them from the server if the user wants to change his language.
    pub available_locale: Vec<Locale>,
}

impl TryFrom<PartialUser> for UserContext {
    type Error = String;

    fn try_from(value: PartialUser) -> Result<Self, Self::Error> {
        let Some(locale) =  Locale::find_by_id(value.locale_id) else {
            return Err(format!("Locale not found for user #{}", value.id));
        };
        Ok(Self {
            translation: Rc::new(locale.translations),
            navlink: Navlink::get_visibility_for_profile(&value.profile),
            available_locale: Locale::get_available_locales(),
            user: value,
        })
    }
}
