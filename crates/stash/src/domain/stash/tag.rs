use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Serialize)]
pub struct Tag(String);

#[derive(Debug, Error)]
pub enum TagError {
    #[error("Invalid character length: {0}")]
    InvalidCharacterLength(usize),
    #[error("Tags must start with a letter")]
    NameMustStartWithLetter,
}

impl FromStr for Tag {
    type Err = TagError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 || s.len() > 15 {
            return Err(TagError::InvalidCharacterLength(s.len()));
        }

        if s.as_bytes()[0].is_ascii_digit() {
            return Err(TagError::NameMustStartWithLetter);
        }

        Ok(Self(s.to_string()))
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let display_name = Tag::from_str(&s).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(display_name)
    }
}
