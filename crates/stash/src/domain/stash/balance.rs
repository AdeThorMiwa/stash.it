use shared::domain::value_objects::{asset::Asset, mula::Mula};

#[derive(Debug, Clone)]
pub struct StashBalance {
    amount: Mula,
    asset: Asset,
}

impl StashBalance {
    pub fn new(asset: &Asset, amount: &Mula) -> Self {
        Self {
            asset: asset.clone(),
            amount: amount.clone(),
        }
    }

    pub fn get_asset(&self) -> &Asset {
        &self.asset
    }

    pub fn update_amount(&mut self, new_amount: &Mula) {
        self.amount = new_amount.clone()
    }
}
