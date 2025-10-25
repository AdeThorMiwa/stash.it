use std::str::FromStr;

use serde::Deserialize;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayName(String);

#[derive(Debug, Error)]
pub enum DisplayNameError {
    #[error("Invalid display name length: {0}")]
    InvalidNameLength(usize),
    #[error("Display name must start with a letter")]
    NameMustStartWithLetter,
}

impl FromStr for DisplayName {
    type Err = DisplayNameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 || s.len() > 25 {
            return Err(DisplayNameError::InvalidNameLength(s.len()));
        }

        if s.as_bytes()[0].is_ascii_digit() {
            return Err(DisplayNameError::NameMustStartWithLetter);
        }

        Ok(Self(s.to_string()))
    }
}

impl ToString for DisplayName {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<'de> Deserialize<'de> for DisplayName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let display_name =
            DisplayName::from_str(&s).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(display_name)
    }
}
