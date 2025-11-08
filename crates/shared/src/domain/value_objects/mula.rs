use crate::domain::value_objects::asset::Asset;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mula {
    amount: u128,
    asset: Asset,
}

impl Mula {
    pub fn new(amount: u128, asset: &Asset) -> Self {
        Self {
            amount,
            asset: asset.to_owned(),
        }
    }

    pub fn get_asset(&self) -> &Asset {
        &self.asset
    }
}
