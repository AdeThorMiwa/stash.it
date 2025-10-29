use chrono::Utc;
use shared::{
    domain::value_objects::{date::Date, mula::Mula, pid::Pid},
    infrastructure::messaging::event::DomainEvent,
};

use crate::domain::stash::status::StashStatus;

#[derive(Debug)]
pub struct StashCreatedEvent {
    stash_id: Pid,
    user_id: Pid,
    created_at: Date,
}

impl StashCreatedEvent {
    pub fn new(stash_id: &Pid, user_id: &Pid) -> Box<Self> {
        Box::new(Self {
            stash_id: stash_id.to_owned(),
            user_id: user_id.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for StashCreatedEvent {
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
pub struct StashStatusUpdatedEvent {
    stash_id: Pid,
    new_status: StashStatus,
    created_at: Date,
}

impl StashStatusUpdatedEvent {
    pub fn new(stash_id: &Pid, new_status: &StashStatus) -> Box<Self> {
        Box::new(Self {
            stash_id: stash_id.to_owned(),
            new_status: new_status.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for StashStatusUpdatedEvent {
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
pub struct StashBalanceUpdatedEvent {
    stash_id: Pid,
    new_balance: Mula,
    created_at: Date,
}

impl StashBalanceUpdatedEvent {
    pub fn new(stash_id: &Pid, new_balance: &Mula) -> Box<Self> {
        Box::new(Self {
            stash_id: stash_id.to_owned(),
            new_balance: new_balance.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for StashBalanceUpdatedEvent {
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
pub struct LedgerEntryCreatedEvent {
    stash_id: Pid,
    entry_id: Pid,
    created_at: Date,
}

impl LedgerEntryCreatedEvent {
    pub fn new(stash_id: &Pid, entry_id: &Pid) -> Box<Self> {
        Box::new(Self {
            stash_id: stash_id.to_owned(),
            entry_id: entry_id.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for LedgerEntryCreatedEvent {
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
