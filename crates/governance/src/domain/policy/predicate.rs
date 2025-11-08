use std::str::FromStr;

use serde::{Deserialize, Serialize};
use shared::infrastructure::types::error::Error;

#[derive(Debug, Clone, Serialize)]
pub struct Predicate(String);

impl ToString for Predicate {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl FromStr for Predicate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl<'de> Deserialize<'de> for Predicate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let predicate_str = String::deserialize(deserializer)?;
        Predicate::from_str(&predicate_str).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}
