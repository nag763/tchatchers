use crate::user::User;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{EncodingKey, Header, encode, Validation, DecodingKey, decode, Algorithm};

#[derive(Serialize, Deserialize)]
pub struct Jwt {
    pub id: i32,
    pub login: String,
    pub name: String,
    pub exp: usize,
}

impl From<User> for Jwt {
    fn from(user: User) -> Jwt {
        Jwt {
            id: user.id,
            login: user.login,
            name: user.name,
            exp: usize::MAX
        }
    }
}

impl Jwt {
    pub fn serialize(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&Header::default(), &self,  &EncodingKey::from_secret(secret.as_ref()))
    }

    pub fn deserialize(token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let token = decode::<Self>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256))?;
        Ok(token.claims)
    }
}
