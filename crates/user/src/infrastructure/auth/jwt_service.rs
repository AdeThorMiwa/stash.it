use crate::{domain::aggregates::user::User, infrastructure::config::Config};
use chrono::{TimeDelta, Utc};
use derive_builder::Builder;
use di::injectable;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use shared::infrastructure::types::{self, Result};
use std::sync::Arc;
use thiserror::Error;

#[injectable]
pub struct JWTService {
    config: Arc<Config>,
}

impl JWTService {
    pub fn generate_token(&self, user: &User) -> Result<String> {
        let claims = ClaimsBuilder::default()
            .exp(Self::get_expiry())
            .sub(user.get_pid().to_string())
            .build()
            .map_err(|_| types::error::Error::ServiceError)?;

        let token = encode(
            &Header::new(Self::get_alg()),
            &claims,
            &EncodingKey::from_secret(self.config.jwt.secret.as_bytes()),
        )
        .map_err(|_| types::error::Error::ServiceError)?;

        Ok(token.to_string())
    }

    pub fn decode_token(&self, token: &str) -> Result<Claims> {
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.config.jwt.secret.as_bytes()),
            &Self::get_validation(),
        )
        .map(|r| r.claims)
        .map_err(|e| {
            println!("error: {e}");
            types::error::Error::ServiceError
        })
    }

    fn get_expiry() -> i64 {
        (Utc::now() + TimeDelta::days(1)).timestamp()
    }

    fn get_alg() -> Algorithm {
        Algorithm::HS512
    }

    fn get_validation() -> Validation {
        let mut validation = Validation::new(Self::get_alg());
        validation.set_audience(&["users.stash.it"]);
        validation.set_issuer(&["auth.stash.it"]);
        validation
    }
}

#[derive(Debug, Error)]
pub enum TokenGenerationError {
    #[error("Failed to build claim: {0}")]
    ClaimBuildFail(String),
    #[error(transparent)]
    EncodingFailed(#[from] jsonwebtoken::errors::Error),
}

impl Into<types::error::Error> for TokenGenerationError {
    fn into(self) -> types::error::Error {
        types::error::Error::ServiceError
    }
}

#[derive(Builder, Debug, Serialize, Deserialize, Clone)]
#[builder(setter(into))]
pub struct Claims {
    /// Audience
    #[builder(default = "users.stash.it".into())]
    aud: String,
    /// Expiration time (as UTC timestamp)
    exp: i64,
    /// Issued at (as UTC timestamp)
    #[builder(default = Utc::now().timestamp())]
    iat: i64,
    /// Issuer
    #[builder(default = "auth.stash.it".into())]
    iss: String,
    /// Not Before (as UTC timestamp)
    #[builder(default = Utc::now().timestamp())]
    nbf: i64,
    /// Subject (whom token refers to). value should be a `Pid`
    pub sub: String,
}
