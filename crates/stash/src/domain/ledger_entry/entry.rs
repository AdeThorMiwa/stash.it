use std::collections::HashMap;

use crate::domain::ledger_entry::entry_type::LedgerEntryType;
use chrono::Utc;
use serde_json::Value;
use shared::domain::value_objects::{date::Date, mula::Mula, pid::Pid};

pub type LedgerEntryMetadata = HashMap<String, Value>;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct LedgerEntry {
    pid: Pid,
    stash_id: Pid,
    entry_type: LedgerEntryType,
    amount: Mula,
    upstream_ref_id: Pid,
    metadata: LedgerEntryMetadata,
    created_at: Date,
}

impl LedgerEntry {
    pub fn new(stash_id: &Pid, entry_type: &LedgerEntryType, amount: &Mula, upstream_ref_id: &Pid) -> Self {
        Self {
            pid: Pid::new(),
            stash_id: stash_id.to_owned(),
            entry_type: entry_type.to_owned(),
            amount: amount.to_owned(),
            upstream_ref_id: upstream_ref_id.to_owned(),
            metadata: HashMap::new(),
            created_at: Utc::now(),
        }
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
