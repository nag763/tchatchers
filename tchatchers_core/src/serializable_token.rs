/// # JWT Serialization and Deserialization
///
/// This module provides a trait for serializing and deserializing JSON Web Tokens (JWTs) using the
/// jsonwebtoken crate. The SerializableToken trait requires that implementing structs be
/// Serialize and Deserialize, and provides methods for encoding and decoding JWTs using a
/// secret key.
///
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub trait SerializableToken
where
    Self: Serialize + Sized + for<'a> Deserialize<'a>,
{
    /// Serializes the JWT into a short string sequence.
    ///
    /// This sequence will then be stored on the front end side and checked on
    /// every secure route to see if the access is legitimate or not.
    ///
    /// # Arguments
    ///
    /// - secret : The secret with which the JWT is serialized. One JWT
    /// serialized with two differents secrets will result in a different string
    fn encode(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
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
    fn decode(token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let token = decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(token.claims)
    }
}
