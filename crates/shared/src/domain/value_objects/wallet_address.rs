use alloy::{hex::FromHexError, primitives::Address};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WalletAddress(String);

pub type WalletAddressError = FromHexError;

impl FromStr for WalletAddress {
    type Err = WalletAddressError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let address = s.parse::<Address>()?;
        Ok(Self(address.to_string()))
    }
}

impl ToString for WalletAddress {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<'de> Deserialize<'de> for WalletAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let address = WalletAddress::from_str(&s).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(address)
    }
}
