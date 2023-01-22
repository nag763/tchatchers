use std::rc::Rc;

use crate::{locale::Locale, navlink::Navlink, translation::Translation, user::PartialUser};

#[derive(Debug, PartialEq, Eq, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AppContext {
    pub user: PartialUser,
    pub translation: Rc<Translation>,
    pub navlink: Vec<Navlink>,
    pub available_locale: Vec<Locale>,
}
