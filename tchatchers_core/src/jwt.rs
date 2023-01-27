// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! The JWT struct modelizes the data that is serialized and shared between
//! the two apps.
//!
//! It is containing important data such as the user and its related
//! informations while hiding the most private ones, that are stored server
//! side.

use crate::user::{PartialUser, User};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// The JWT structure, holding the data that is shared between the front and
/// the back.
#[derive(Serialize, Deserialize)]
pub struct Jwt {
    /// User related informations.
    pub user: PartialUser,
    /// The expiracy time on which the JWT expires.
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
    /// Serializes the JWT into a short string sequence.
    ///
    /// This sequence will then be stored on the front end side and checked on
    /// every secure route to see if the access is legitimate or not.
    ///
    /// # Arguments
    ///
    /// - secret : The secret with which the JWT is serialized. One JWT
    /// serialized with two differents secrets will result in a different string
    /// , securing the token.
    pub fn serialize(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }

    /// Deserializes a token and transform it into a JWT.
    ///
    /// This method should mostly be used on server side to get the information
    /// about the user that is calling a webservice, or as a guard to secure
    /// routes.
    ///
    /// The deserialization will fail if the secret used to serialize the token
    /// differs from the one used to deserialize the structure.
    ///
    /// # Arguments
    ///
    /// - token : The token to deserialize.
    pub fn deserialize(token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let token = decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(token.claims)
    }
}
