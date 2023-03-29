// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! The JWT struct modelizes the data that is serialized and shared between
//! the two apps.
//!
//! It is containing important data such as the user and its related
//! informations while hiding the most private ones, that are stored server
//! side.

use crate::common::AUTHORIZATION_TOKEN_EXPIRACY_TIME;
use crate::serializable_token::SerializableToken;
use crate::{profile::Profile, user::User};
use serde::{Deserialize, Serialize};

/// The JWT structure, holding the data that is shared between the front and
/// the back.
#[derive(Serialize, Deserialize)]
pub struct AuthorizationToken {
    /// User id.
    pub user_id: i32,
    /// User profile.
    pub user_profile: Profile,
    /// The expiracy time on which the JWT expires.
    pub exp: i64,
}

impl From<User> for AuthorizationToken {
    fn from(user: User) -> AuthorizationToken {
        AuthorizationToken {
            user_id: user.id,
            user_profile: user.profile,
            exp: (chrono::Utc::now() + *AUTHORIZATION_TOKEN_EXPIRACY_TIME).timestamp(),
        }
    }
}

impl SerializableToken for AuthorizationToken {}
