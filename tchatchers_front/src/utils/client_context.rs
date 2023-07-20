use std::rc::Rc;

use tchatchers_core::{
    locale::{Locale, TranslationMap},
    navlink::Navlink,
    user::PartialUser,
};
use yew::UseStateHandle;

#[derive(Clone, Debug, PartialEq)]
pub struct ClientContext {
    pub bearer: UseStateHandle<Option<String>>,
    /// The user currently being logged in.
    pub user: UseStateHandle<Option<PartialUser>>,
    /// The user's locale.
    pub locale: Rc<Option<Locale>>,
    /// The list of navigation links he can access to.
    ///
    /// This depends on the user's profile.
    pub navlink: Rc<Vec<Navlink>>,
    /// The locales available.
    ///
    /// Mainly used to not request them from the server if the user wants to change his language.
    pub available_locale: Vec<Locale>,
    /// The translations that will be used.
    pub translation: Rc<TranslationMap>,
}
