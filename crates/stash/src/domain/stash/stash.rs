use crate::domain::stash::{balance::StashBalance, name::StashName, status::StashStatus, tag::Tag};
use chrono::Utc;
use serde_json::Value;
use shared::domain::value_objects::{asset::Asset, date::Date, mula::Mula, pid::Pid};
use std::collections::HashMap;

pub type StashMetadata = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub struct Stash {
    pid: Pid,
    user_id: Pid,
    name: StashName,
    status: StashStatus,
    tags: Vec<Tag>,
    balances: Vec<StashBalance>,
    metadata: StashMetadata,
    #[allow(dead_code)]
    created_at: Date,
    updated_at: Date,
}

impl Stash {
    pub fn new(user_id: &Pid, name: &StashName, tags: &Vec<Tag>) -> Self {
        Self {
            pid: Pid::new(),
            user_id: user_id.clone(),
            name: name.clone(),
            status: StashStatus::ACTIVE,
            tags: tags.clone(),
            balances: Vec::new(),
            metadata: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn get_pid(&self) -> &Pid {
        &self.pid
    }

    pub fn get_name(&self) -> &StashName {
        &self.name
    }

    pub fn get_user_id(&self) -> &Pid {
        &self.user_id
    }

    pub fn get_status(&self) -> &StashStatus {
        &self.status
    }

    pub fn get_tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn get_balances(&self) -> &Vec<StashBalance> {
        &self.balances
    }

    pub fn get_metadata(&self) -> &StashMetadata {
        &self.metadata
    }

    pub fn update_name(&mut self, new_name: &StashName) {
        self.name = new_name.clone();
        self.updated_at = Utc::now();
    }

    pub fn update_status(&mut self, new_status: &StashStatus) {
        self.status = new_status.clone();
        self.updated_at = Utc::now();
    }

    pub fn update_balance(&mut self, asset: &Asset, new_balance: &Mula) {
        self.balances = self
            .balances
            .iter_mut()
            .map(|balance| {
                if balance.get_asset().eq(asset) {
                    balance.update_amount(new_balance);
                }

                balance.clone()
            })
            .collect();
        self.updated_at = Utc::now();
    }
}
