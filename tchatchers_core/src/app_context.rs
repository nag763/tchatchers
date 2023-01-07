use crate::{translation::Translation, user::PartialUser};

#[derive(Debug, PartialEq, Eq, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AppContext {
    pub user: PartialUser,
    pub translation: Translation,
}
