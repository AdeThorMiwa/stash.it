use crate::domain::{
    events::{StashBalanceUpdated, StashCreated, StashStatusUpdated},
    stash::{name::StashName, status::StashStatus, tag::Tag},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{
    domain::{
        entity::Entity,
        value_objects::{date::Date, mula::Mula, pid::Pid},
    },
    infrastructure::messaging::event::DomainEvent,
};
use std::{collections::HashMap, sync::Arc};

pub type StashMetadata = HashMap<String, Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stash {
    pid: Pid,
    owner_id: Pid,
    name: StashName,
    status: StashStatus,
    tags: Vec<Tag>,
    balances: Vec<Mula>,
    metadata: StashMetadata,
    #[allow(dead_code)]
    created_at: Date,
    updated_at: Date,
    #[serde(skip)]
    events: Vec<Arc<dyn DomainEvent>>,
}

impl Stash {
    pub fn new(owner_id: &Pid, name: &StashName, tags: &Vec<Tag>) -> Self {
        let pid = Pid::new();
        Self {
            pid: pid.to_owned(),
            owner_id: owner_id.to_owned(),
            name: name.clone(),
            status: StashStatus::ACTIVE,
            tags: tags.clone(),
            balances: Vec::new(),
            metadata: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            events: Self::get_initial_events(&pid, owner_id),
        }
    }

    fn get_initial_events(pid: &Pid, owner_id: &Pid) -> Vec<Arc<dyn DomainEvent>> {
        let mut events: Vec<Arc<dyn DomainEvent>> = Vec::new();
        events.push(StashCreated::new(&pid, owner_id));
        events
    }

    pub fn get_pid(&self) -> &Pid {
        &self.pid
    }

    pub fn get_name(&self) -> &StashName {
        &self.name
    }

    pub fn get_owner_id(&self) -> &Pid {
        &self.owner_id
    }

    pub fn get_status(&self) -> &StashStatus {
        &self.status
    }

    pub fn get_tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn get_balances(&self) -> &Vec<Mula> {
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
        self.events.push(StashStatusUpdated::new(self.get_pid(), new_status));
    }

    pub fn update_balance(&mut self, new_balance: &Mula) {
        if let Some(balance) = self.balances.iter_mut().find(|b| b.get_asset().eq(new_balance.get_asset())) {
            *balance = new_balance.clone();
        } else {
            self.balances.push(new_balance.clone());
        }
        self.updated_at = Utc::now();
        self.events.push(StashBalanceUpdated::new(self.get_pid(), new_balance));
    }
}

impl Entity for Stash {
    fn drain_events(&mut self) -> Vec<Arc<dyn DomainEvent>> {
        self.events.drain(..).collect()
    }
}
