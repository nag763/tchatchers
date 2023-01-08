use crate::{navlink::Navlink, translation::Translation, user::PartialUser};

#[derive(Debug, PartialEq, Eq, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AppContext {
    pub user: PartialUser,
    pub translation: Translation,
    pub navlink: Vec<Navlink>,
}
