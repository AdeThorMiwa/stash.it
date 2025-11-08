use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct StashName(String);

#[derive(Debug, Error)]
pub enum StashNameError {
    #[error("Invalid character length: {0}")]
    InvalidCharacterLength(usize),
    #[error("Stash name must start with a letter")]
    NameMustStartWithLetter,
}

impl FromStr for StashName {
    type Err = StashNameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 || s.len() > 15 {
            return Err(StashNameError::InvalidCharacterLength(s.len()));
        }

        if s.as_bytes()[0].is_ascii_digit() {
            return Err(StashNameError::NameMustStartWithLetter);
        }

        Ok(Self(s.to_string()))
    }
}

impl ToString for StashName {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<'de> Deserialize<'de> for StashName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let display_name = StashName::from_str(&s).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(display_name)
    }
}
