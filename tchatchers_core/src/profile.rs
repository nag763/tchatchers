use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Hash, derive_more::Display,
)]
#[cfg_attr(feature = "back", derive(sqlx::Type))]
#[repr(i32)]
pub enum Profile {
    #[default]
    User = 1,
    Moderator = 2,
    Admin = 3,
}

impl Profile {
    pub fn iterator() -> impl Iterator<Item = Self> {
        [Profile::User, Profile::Moderator, Profile::Admin].into_iter()
    }
}
