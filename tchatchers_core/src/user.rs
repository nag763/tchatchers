//! A user modelizes an authenticated client of the application.
//!
//! The user is declined under different structs so that only the revelant data
//! is shared between processed and components.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::common::RE_LIMITED_CHARS;
use crate::jwt::Jwt;
use crate::profile::Profile;
use crate::timezone::Timezone;
#[cfg(feature = "back")]
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
#[cfg(feature = "back")]
use sqlx::postgres::PgQueryResult;
#[cfg(feature = "back")]
use sqlx::FromRow;
#[cfg(feature = "back")]
use sqlx::PgPool;
use validator::Validate;
use validator::ValidationError;

lazy_static! {
    static ref RE_ONE_LOWERCASE_CHAR: Regex = Regex::new(r"[a-z]+").unwrap();
    static ref RE_ONE_NUMBER: Regex = Regex::new(r"[0-9]+").unwrap();
    static ref RE_ONE_UPPERCASE_CHAR: Regex = Regex::new(r"[A-Z]+").unwrap();
}

/// The in base structure, which should never be shared between components and
/// apps.
#[derive(Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "back", derive(FromRow))]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The in base id, unique.
    pub id: i32,
    /// The login of the user, should also be unique.
    pub login: String,
    /// The user password, should always be encrypted and secured.
    pub password: String,
    /// Whether the user is authorized to connect to the different services.
    ///
    /// If set to false, the user shouldn't be allowed to connect to the apps.
    pub is_authorized: bool,
    /// The name of the user, should be displayed on front end services.
    pub name: String,
    /// The profile picture of the user.
    pub pfp: Option<String>,
    /// The locale associated with the user.
    pub locale_id: i32,
    #[cfg_attr(feature = "back", sqlx(rename = "profile_id"))]
    pub profile: Profile,
    #[cfg_attr(feature = "back", sqlx(flatten))]
    pub timezone: crate::timezone::Timezone,
}

#[cfg(feature = "back")]
impl User {
    /// Find a user by ID in the database.
    ///
    /// This shouldn't be used to identify users.
    ///
    /// # Arguments
    ///
    /// - id : The id of the user we are looking for.
    /// - pool : The pool of connection.
    pub async fn find_by_id(id: i32, pool: &PgPool) -> Option<Self> {
        sqlx::query_as("SELECT * FROM CHATTER WHERE id=$1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .unwrap()
    }

    /// Look up whether a login exists in the database.
    ///
    /// # Arguments
    ///
    /// - login : The login to look up.
    pub async fn login_exists(login: &str, pool: &PgPool) -> bool {
        let row: (bool,) = sqlx::query_as("SELECT COUNT(id)!=0 FROM CHATTER WHERE login=$1")
            .bind(login)
            .fetch_one(pool)
            .await
            .unwrap();
        row.0
    }

    pub async fn delete_one(id: i32, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM CHATTER WHERE id=$1")
            .bind(id)
            .execute(pool)
            .await
    }
}

impl From<Jwt> for User {
    fn from(jwt: Jwt) -> User {
        User {
            id: jwt.user.id,
            login: jwt.user.login,
            name: jwt.user.name,
            ..User::default()
        }
    }
}

/// Structure mostly used to share the data between the applications.
///
/// It is containing limited data, which is convenient and secure during
/// exchanges. Thus, this is the struct used in JWT.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct PartialUser {
    /// In base id of the user.
    pub id: i32,
    /// Login of the user.
    pub login: String,
    /// Name of the user.
    pub name: String,
    /// Profile picture of the user.
    pub pfp: Option<String>,
    /// The locale associated with the user.
    pub locale_id: i32,
    #[cfg_attr(feature = "back", sqlx(rename = "profile_id"))]
    // The profile of the user
    pub profile: Profile,
    #[cfg_attr(feature = "back", sqlx(rename = "profile_id"))]
    #[cfg_attr(feature = "back", sqlx(flatten))]
    pub timezone: crate::timezone::Timezone,
}

impl From<User> for PartialUser {
    fn from(user: User) -> PartialUser {
        PartialUser {
            id: user.id,
            login: user.login,
            name: user.name,
            pfp: user.pfp,
            locale_id: user.locale_id,
            profile: user.profile,
            timezone: user.timezone,
        }
    }
}

fn password_strengh(password: &str) -> Result<(), ValidationError> {
    if !RE_ONE_LOWERCASE_CHAR.is_match(password)
        || !RE_ONE_UPPERCASE_CHAR.is_match(password)
        || !RE_ONE_NUMBER.is_match(password)
    {
        Err(ValidationError::new("security_constraints_not_matched"))
    } else {
        Ok(())
    }
}

/// Structure used only to create new DB entities.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsertableUser {
    /// The user log in.
    #[validate(
        length(min = 3, max = 32),
        regex(path = "RE_LIMITED_CHARS", code = "limited_chars")
    )]
    pub login: String,
    /// The user password, should be raw prior being insert.
    #[validate(
        length(min = 8, max = 128),
        custom(
            function = "password_strengh",
            code = "security_constraints_not_matched"
        )
    )]
    pub password: String,
    /// The name of the user.
    #[validate(
        length(min = 3, max = 16),
        regex(path = "RE_LIMITED_CHARS", code = "limited_chars")
    )]
    pub name: String,
}

#[cfg(feature = "back")]
impl InsertableUser {
    /// Inserts the user in the database.
    ///
    /// # Arguments
    ///
    /// - pool : The connection pool.
    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(self.password.as_bytes(), &salt, &config).unwrap();
        sqlx::query("INSERT INTO CHATTER(login, password, name) VALUES ($1,$2,$3)")
            .bind(&self.login)
            .bind(&hash)
            .bind(&self.name)
            .execute(pool)
            .await
    }
}

/// The updatabale structure, should only be used to update a db entity.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdatableUser {
    pub id: i32,
    #[validate(
        length(min = 3, max = 16),
        regex(path = "RE_LIMITED_CHARS", code = "limited_chars")
    )]
    pub name: String,
    pub pfp: Option<String>,
    pub locale_id: i32,
    pub timezone: Timezone,
}

#[cfg(feature = "back")]
impl UpdatableUser {
    /// Updates the user in the database.
    ///
    /// # Arguments
    ///
    /// - pool : The connection pool.
    pub async fn update(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("UPDATE CHATTER SET name=$1, pfp=$2, locale_id=$3, tz_name=$4, tz_offset=$5 WHERE id=$6")
            .bind(&self.name)
            .bind(&self.pfp)
            .bind(self.locale_id)
            .bind(&self.timezone.tz_name)
            .bind(self.timezone.tz_offset)
            .bind(self.id)
            .execute(pool)
            .await
    }
}

/// Structure used to authenticate a user.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Validate)]
pub struct AuthenticableUser {
    /// The login of the user, has to exist in database.
    pub login: String,
    /// The password of the user, has to be encrypted.
    pub password: String,
}

#[cfg(feature = "back")]
impl AuthenticableUser {
    /// Authenticate the user in the database.
    ///
    /// It will return the full structure if the authentication is successful.
    ///
    /// # Arguments
    ///
    /// - pool : The connection pool.
    pub async fn authenticate(&self, pool: &PgPool) -> Option<User> {
        let user: User =
            sqlx::query_as("SELECT * FROM CHATTER WHERE login=$1 AND is_authorized=true")
                .bind(&self.login)
                .fetch_optional(pool)
                .await
                .unwrap()?;
        match argon2::verify_encoded(&user.password, self.password.as_bytes()).unwrap() {
            true => Some(user),
            false => None,
        }
    }
}
