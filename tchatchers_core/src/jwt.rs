use crate::user::{PartialUser, User};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Jwt {
    pub user: PartialUser,
    pub exp: usize,
}

impl From<User> for Jwt {
    fn from(user: User) -> Jwt {
        Jwt {
            user: user.into(),
            exp: usize::MAX,
        }
    }
}

impl Jwt {
    pub fn serialize(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }

    pub fn deserialize(token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let token = decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(token.claims)
    }
}
