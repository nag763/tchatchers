use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use chrono::{DateTime, Duration, Utc};
use derivative::Derivative;
use derive_more::Display;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

use crate::serializable_token::SerializableToken;

#[derive(Debug, Default, Serialize, Deserialize, Display, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FunctionalTokenType {
    #[default]
    ValidatingMail,
}

impl FunctionalTokenType {
    fn get_valid_time(&self) -> Duration {
        match self {
            Self::ValidatingMail => chrono::Duration::days(2),
        }
    }
}

#[derive(Serialize, Deserialize, Derivative, Hash, PartialEq, Eq)]
pub struct FunctionalToken {
    pub user_id: i32,
    pub token_type: FunctionalTokenType,
    pub emitted_on: DateTime<Utc>,
    pub exp: i64,
}

impl SerializableToken for FunctionalToken {}

impl FunctionalToken {
    pub fn is_expired(&self) -> bool {
        self.exp < chrono::offset::Utc::now().timestamp()
    }

    fn get_key(&self) -> String {
        format!("{}{}", self.user_id, self.token_type)
    }

    pub fn new(user_id: i32, token_type: FunctionalTokenType) -> FunctionalToken {
        let now = chrono::offset::Utc::now();

        Self {
            user_id,
            token_type,
            emitted_on: now,
            exp: (now + token_type.get_valid_time()).timestamp(),
        }
    }

    pub async fn emit(&self, con: &mut redis::aio::Connection) -> Result<bool, redis::RedisError> {
        let mut default_hasher = DefaultHasher::default();

        self.hash(&mut default_hasher);
        let res = con
            .set_ex(
                self.get_key(),
                default_hasher.finish(),
                self.token_type
                    .get_valid_time()
                    .num_seconds()
                    .try_into()
                    .unwrap(),
            )
            .await?;
        Ok(res)
    }

    pub async fn is_latest(
        &self,
        con: &mut redis::aio::Connection,
    ) -> Result<bool, redis::RedisError> {
        let mut default_hasher = DefaultHasher::default();
        self.hash(&mut default_hasher);
        let stored_token: Option<u64> = con.get(self.get_key()).await?;
        Ok(matches!(stored_token, Some(v) if v == default_hasher.finish()))
    }

    pub async fn consume(
        &self,
        con: &mut redis::aio::Connection,
    ) -> Result<bool, redis::RedisError> {
        let res = con.del(self.get_key()).await?;
        Ok(res)
    }
}
