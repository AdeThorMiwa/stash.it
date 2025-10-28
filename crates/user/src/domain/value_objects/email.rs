use email_address::{EmailAddress as Address, Error};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailAddress(Address);

pub type EmailError = Error;

impl FromStr for EmailAddress {
    type Err = EmailError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Address::from_str(s).map(|address| Self(address))
    }
}

impl ToString for EmailAddress {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let email_address = EmailAddress::from_str(&s).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(email_address)
    }
}
