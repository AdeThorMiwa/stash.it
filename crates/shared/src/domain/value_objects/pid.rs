use std::str::FromStr;

use serde::Deserialize;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pid(Uuid);

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PidError {
    #[error("Invalid Pid format")]
    ParseError,
}

impl Pid {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl FromStr for Pid {
    type Err = PidError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(|id| Self(id)).map_err(|_| PidError::ParseError)
    }
}

impl<'de> Deserialize<'de> for Pid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let pid = Pid::from_str(&s).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(pid)
    }
}

impl ToString for Pid {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
