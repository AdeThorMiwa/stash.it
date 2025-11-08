use std::sync::Arc;

use chrono::Utc;
use shared::{
    domain::value_objects::{date::Date, mula::Mula, pid::Pid},
    infrastructure::messaging::event::DomainEvent,
};

use crate::domain::stash::status::StashStatus;

#[derive(Debug)]
pub struct StashCreated {
    pub stash_id: Pid,
    pub owner_id: Pid,
    pub created_at: Date,
}

impl StashCreated {
    #[must_use]
    pub fn new(stash_id: &Pid, owner_id: &Pid) -> Arc<Self> {
        Arc::new(Self {
            stash_id: stash_id.to_owned(),
            owner_id: owner_id.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for StashCreated {
    fn event_type(&self) -> &str {
        "StashCreated"
    }

    fn aggregate_id(&self) -> Pid {
        self.stash_id.clone()
    }

    fn occurred_at(&self) -> Date {
        self.created_at.clone()
    }
}

#[derive(Debug)]
pub struct StashStatusUpdated {
    stash_id: Pid,
    pub new_status: StashStatus,
    created_at: Date,
}

impl StashStatusUpdated {
    pub fn new(stash_id: &Pid, new_status: &StashStatus) -> Arc<Self> {
        Arc::new(Self {
            stash_id: stash_id.to_owned(),
            new_status: new_status.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for StashStatusUpdated {
    fn event_type(&self) -> &str {
        "StashStatusUpdated"
    }

    fn aggregate_id(&self) -> Pid {
        self.stash_id.clone()
    }

    fn occurred_at(&self) -> Date {
        self.created_at.clone()
    }
}

#[derive(Debug)]
pub struct StashBalanceUpdated {
    stash_id: Pid,
    pub new_balance: Mula,
    created_at: Date,
}

impl StashBalanceUpdated {
    pub fn new(stash_id: &Pid, new_balance: &Mula) -> Arc<Self> {
        Arc::new(Self {
            stash_id: stash_id.to_owned(),
            new_balance: new_balance.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for StashBalanceUpdated {
    fn event_type(&self) -> &str {
        "StashBalanceUpdated"
    }

    fn aggregate_id(&self) -> Pid {
        self.stash_id.clone()
    }

    fn occurred_at(&self) -> Date {
        self.created_at.clone()
    }
}

#[derive(Debug)]
pub struct LedgerEntryCreated {
    stash_id: Pid,
    pub entry_id: Pid,
    created_at: Date,
}

impl LedgerEntryCreated {
    pub fn new(stash_id: &Pid, entry_id: &Pid) -> Arc<Self> {
        Arc::new(Self {
            stash_id: stash_id.to_owned(),
            entry_id: entry_id.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for LedgerEntryCreated {
    fn event_type(&self) -> &str {
        "LedgerEntryCreated"
    }

    fn aggregate_id(&self) -> Pid {
        self.stash_id.clone()
    }

    fn occurred_at(&self) -> Date {
        self.created_at.clone()
    }
}
