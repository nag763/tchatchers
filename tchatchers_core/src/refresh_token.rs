/// This module contains the implementation of RefreshToken which is used to represent a refresh token
/// that is used to renew access tokens used by the client.
///
/// It is derived from Derivative trait which allows it to derive some traits from others such as
/// Default, Clone, Copy, Serialize, Deserialize.
///
/// It contains the following functions:
///
/// - from: used to convert from a user object to a RefreshToken object.
/// - new: used to create a new instance of RefreshToken.
/// - renew: used to renew the instance of RefreshToken.
/// - store_in_jar: used to store the RefreshToken object in a cookie jar.
///
/// The following functions are available only when the back feature is enabled:
///
/// - store_in_jar: used to store the RefreshToken object in a cookie jar.
///
/// The constants available in this module are:
///
/// - REFRESH_TOKEN_EXPIRACY_TIME: represents the duration for the refresh token to expire.
///
/// The struct available in this module is:
///
/// - RefreshToken: represents the refresh token that is used to renew access tokens used by the client.
///   It has the following fields:
/// - user_id: represents the id of the user.
/// - exp: represents the expiration time of the refresh token.
/// - session_only: represents whether the refresh token should be used for a session only.
///
/// It also implements the SerializableToken trait, which allows it to be serialized as a token.
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{common::REFRESH_TOKEN_EXPIRACY_TIME, serializable_token::SerializableToken};

#[cfg(feature = "back")]
use axum_extra::extract::cookie::{Cookie, CookieJar};
#[cfg(feature = "back")]
use redis::AsyncCommands;
#[cfg(feature = "back")]
const REFRESH_TOKEN_PATH: &str = "refresh_token";
#[cfg(feature = "back")]
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// Represents a refresh token used for authentication.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Derivative)]
#[cfg_attr(feature = "back", derive(Hash))]
#[derivative(Default)]
pub struct RefreshToken {
    /// The ID of the user associated with the token.
    pub user_id: i32,
    /// The expiration timestamp of the token, in seconds since the UNIX epoch.
    #[derivative(Default(
        value = "(chrono::Utc::now() + *REFRESH_TOKEN_EXPIRACY_TIME.get_or_init(|| chrono::Duration::try_minutes(15).unwrap())).timestamp()"
    ))]
    pub exp: i64,
    /// Whether the token should be limited to the current session only.
    #[derivative(Default(value = "true"))]
    pub session_only: bool,
    /// The token family of the token, used to detect refresh token reusage.
    #[derivative(Default(value = "Uuid::new_v4()"))]
    pub token_family: Uuid,
}

#[cfg(feature = "back")]
impl RefreshToken {
    /// Creates a new `RefreshToken` with the specified user ID and session-only flag.
    ///
    /// # Arguments
    ///
    /// - `user_id`: The ID of the user associated with the token.
    /// - `session_only`: Whether the token should be limited to the current session only.
    pub fn new(user_id: i32, session_only: bool) -> Self {
        Self {
            user_id,
            session_only,
            ..Self::default()
        }
    }

    /// Renews this token, returning a new token with the same user ID and session-only flag,
    /// but with a new expiration timestamp.
    pub fn renew(&self) -> Self {
        Self {
            user_id: self.user_id,
            session_only: self.session_only,
            token_family: self.token_family,
            ..Self::default()
        }
    }

    /// Encodes this token as a JWT string and stores it in a `CookieJar`.
    ///
    /// # Arguments
    ///
    /// - `secret`: The secret key used to sign the token.
    /// - `jar`: The `CookieJar` in which to store the token.
    pub fn store_in_jar(
        &self,
        secret: &str,
        jar: CookieJar,
    ) -> Result<CookieJar, jsonwebtoken::errors::Error> {
        let mut cookie = Cookie::new(REFRESH_TOKEN_PATH, self.encode(secret)?);
        cookie.set_path("/");
        cookie.set_name(REFRESH_TOKEN_PATH);
        match self.session_only {
            true => cookie.set_expires(None),
            false => {
                let now = time::OffsetDateTime::UNIX_EPOCH;
                let expiration = now + time::Duration::seconds(self.exp);
                cookie.set_expires(expiration)
            }
        }
        cookie.set_secure(true);
        cookie.set_http_only(true);
        Ok(jar.add(cookie))
    }

    /// Set this token as the head token for its family in Redis.
    ///
    /// # Arguments
    ///
    /// * `con` - A mutable reference to a Redis connection to execute the Redis command.
    ///
    /// # Returns
    ///
    /// Returns a boolean indicating whether the Redis command executed successfully.
    pub async fn set_as_head_token(
        &self,
        con: &mut redis::aio::MultiplexedConnection,
    ) -> Result<bool, redis::RedisError> {
        let mut default_hasher = DefaultHasher::default();

        self.hash(&mut default_hasher);
        con.set_ex(
            self.token_family.to_string(),
            default_hasher.finish(),
            REFRESH_TOKEN_EXPIRACY_TIME
                .get_or_init(|| chrono::Duration::try_minutes(15).unwrap())
                .num_seconds()
                .try_into()
                .unwrap(),
        )
        .await
    }

    /// Check whether this token is the head token for its family in Redis.
    ///
    /// # Arguments
    ///
    /// * `con` - A mutable reference to a Redis connection to execute the Redis command.
    ///
    /// # Returns
    ///
    /// Returns a boolean indicating whether this token is the head token for its family in Redis.
    pub async fn is_head_token(
        &self,
        con: &mut redis::aio::MultiplexedConnection,
    ) -> Result<bool, redis::RedisError> {
        let mut default_hasher = DefaultHasher::default();
        self.hash(&mut default_hasher);

        let head_token: Option<u64> = con.get(self.token_family.to_string()).await?;
        Ok(matches!(head_token, Some(v) if v == default_hasher.finish()))
    }

    /// Delete this token family from Redis.
    ///
    /// # Arguments
    ///
    /// * `con` - A Redis client to execute the Redis command.
    ///
    /// # Returns
    ///
    /// Returns a boolean indicating whether the Redis command executed successfully.
    pub async fn revoke_family(
        &self,
        con: &mut redis::aio::MultiplexedConnection,
    ) -> Result<bool, redis::RedisError> {
        // Execute a Redis `DEL` command to delete this token family.
        // Returns a boolean indicating whether the Redis command executed successfully.
        con.del(self.token_family.to_string()).await
    }
}

impl SerializableToken for RefreshToken {}
