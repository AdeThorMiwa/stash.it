use crate::domain::value_objects::wallet_address::WalletAddress;

#[derive(Clone, Debug, Eq)]
pub struct Asset {
    name: String,
    symbol: String,
    network: String,
    address: Option<WalletAddress>,
    decimals: u8,
    display_decimals: u8,
}

impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol && self.network == other.network
    }
}
