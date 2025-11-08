use std::{collections::HashMap, sync::Arc};

use crate::domain::{events::LedgerEntryCreated, ledger_entry::entry_type::LedgerEntryType};
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

pub type LedgerEntryMetadata = HashMap<String, Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(unused)]
pub struct LedgerEntry {
    pid: Pid,
    stash_id: Pid,
    entry_type: LedgerEntryType,
    amount: Mula,
    upstream_ref_id: Pid,
    metadata: LedgerEntryMetadata,
    created_at: Date,
    #[serde(skip)]
    events: Vec<Arc<dyn DomainEvent>>,
}

impl LedgerEntry {
    pub fn new(stash_id: &Pid, entry_type: &LedgerEntryType, amount: &Mula, upstream_ref_id: &Pid) -> Self {
        let pid = Pid::new();
        Self {
            pid: pid.to_owned(),
            stash_id: stash_id.to_owned(),
            entry_type: entry_type.to_owned(),
            amount: amount.to_owned(),
            upstream_ref_id: upstream_ref_id.to_owned(),
            metadata: HashMap::new(),
            created_at: Utc::now(),
            events: Self::get_initial_events(&pid, &stash_id),
        }
    }

    fn get_initial_events(pid: &Pid, stash_id: &Pid) -> Vec<Arc<dyn DomainEvent>> {
        let mut events: Vec<Arc<dyn DomainEvent>> = Vec::new();
        events.push(LedgerEntryCreated::new(&stash_id, pid));
        events
    }

    pub fn get_pid(&self) -> &Pid {
        &self.pid
    }

    pub fn get_stash_id(&self) -> &Pid {
        &self.stash_id
    }

    pub fn get_type(&self) -> &LedgerEntryType {
        &self.entry_type
    }

    pub fn get_amount(&self) -> &Mula {
        &self.amount
    }

    pub fn get_upstream_ref_id(&self) -> &Pid {
        &self.upstream_ref_id
    }
}

impl Entity for LedgerEntry {
    fn drain_events(&mut self) -> Vec<Arc<dyn DomainEvent>> {
        self.events.drain(..).collect()
    }
}
