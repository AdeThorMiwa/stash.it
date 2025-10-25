use crate::{domain::aggregates::user::User, infrastructure::config::Config};
use chrono::{TimeDelta, Utc};
use derive_builder::Builder;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use macros::inject;
use shared::infrastructure::types::Result;
use std::collections::HashMap;
use thiserror::Error;

#[inject]
pub struct JWTService {
    config: Config,
}

impl JWTService {
    pub fn generate_token(&self, user: &User) -> Result<String> {
        let claims = ClaimsBuilder::default()
            .exp(Self::get_expiry())
            .sub(user.get_pid())
            .build()
            .map_err(|e| TokenGenerationError::ClaimBuildFail(e.to_string()));

        let token = encode(
            &Header::new(Self::get_alg()),
            &claims,
            &EncodingKey::from_base64_secret(self.config.jwt.secret.as_str()),
        )
        .map_err(TokenGenerationError::EncodingFailed)?;

        Ok(token)
    }

    fn get_expiry() -> usize {
        (Utc::now() + TimeDelta::days(1)).timestamp()
    }

    fn get_alg() -> Algorithm {
        Algorithm::HS512
    }
}

#[derive(Debug, Error)]
pub enum TokenGenerationError {
    #[error("Failed to build claim: {0}")]
    ClaimBuildFail(String),
    #[error(transparent)]
    EncodingFailed(#[from] jsonwebtoken::errors::Error),
}

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct Claims {
    /// Audience
    #[builder(default = "users.stash.it")]
    aud: String,
    /// Expiration time (as UTC timestamp)
    exp: usize,
    /// Issued at (as UTC timestamp)
    #[builder(default = Utc::now().timestamp())]
    iat: usize,
    /// Issuer
    #[builder(default = "auth.stash.it")]
    iss: String,
    /// Not Before (as UTC timestamp)
    #[builder(default = Utc::now().timestamp())]
    nbf: usize,
    /// Subject (whom token refers to). value should be a `Pid`
    sub: String,
}
