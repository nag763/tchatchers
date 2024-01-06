// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! This crate is used to list the different profiles of an user.

use serde::{Deserialize, Serialize};

/// The profile linked with a user.
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Hash,
    derive_more::Display,
    PartialOrd,
)]
#[cfg_attr(
    any(feature = "back", feature = "cli", feature = "async"),
    derive(sqlx::Type)
)]
#[repr(i32)]
pub enum AuthorizationStatus {
    /// Simple user, with little rights.
    #[default]
    Deactivated = 1,
    /// Moderator, can perform some administration actions.
    PendingVerification = 10,
    /// Moderator, can perform some administration actions.
    UnverifiedAuthorized = 11,
    /// Administrator, can perform all the administrative actions.
    Verified = 20,
    /// Moderator, can perform some administration actions.
    AuthorizedByAdmin = 21,
}

impl AuthorizationStatus {
    pub fn is_authorized(&self) -> bool {
        match self {
            AuthorizationStatus::Verified
            | AuthorizationStatus::AuthorizedByAdmin
            | AuthorizationStatus::UnverifiedAuthorized => true,
            _ => false,
        }
    }

    pub fn is_deactivated(&self) -> bool {
        match self {
            AuthorizationStatus::Deactivated => true,
            _ => false,
        }
    }

    /// Returns an iterator over all the variants of the Profile enum.
    pub fn iterator() -> impl Iterator<Item = Self> {
        Self::options().into_iter()
    }

    /// Returns the profile options.
    pub fn options() -> [AuthorizationStatus; 5] {
        [
            AuthorizationStatus::Deactivated,
            AuthorizationStatus::PendingVerification,
            AuthorizationStatus::UnverifiedAuthorized,
            AuthorizationStatus::Verified,
            AuthorizationStatus::AuthorizedByAdmin,
        ]
    }
}
