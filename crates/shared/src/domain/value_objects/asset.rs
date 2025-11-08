use crate::domain::value_objects::wallet_address::WalletAddress;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub symbol: String,
    pub network: String,
    pub address: Option<WalletAddress>,
    pub decimals: u8,
    pub display_decimals: u8,
}

impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol && self.network == other.network
    }
}

#[cfg(feature = "testing")]
impl Asset {
    pub fn usdt() -> Self {
        Self {
            name: "USD Tether".to_owned(),
            network: "ethereum".to_owned(),
            symbol: "USDT".to_owned(),
            address: None,
            decimals: 18,
            display_decimals: 10,
        }
    }
}
