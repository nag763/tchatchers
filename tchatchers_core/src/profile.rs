// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! This crate is used to list the different profiles of an user.

use serde::{Deserialize, Serialize};

/// The profile linked with a user.
#[derive(
    Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Hash, derive_more::Display,
)]
#[cfg_attr(any(feature = "back", feature = "cli"), derive(sqlx::Type))]
#[repr(i32)]
pub enum Profile {
    /// Simple user, with little rights.
    #[default]
    User = 1,
    /// Moderator, can perform some administration actions.
    Moderator = 2,
    /// Administrator, can perform all the administrative actions.
    Admin = 3,
}

impl Profile {
    /// Returns an iterator over all the variants of the Profile enum.
    pub fn iterator() -> impl Iterator<Item = Self> {
        [Profile::User, Profile::Moderator, Profile::Admin].into_iter()
    }
}
