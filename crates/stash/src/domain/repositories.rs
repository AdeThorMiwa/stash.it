use async_trait::async_trait;
use derive_builder::Builder;
use shared::{domain::value_objects::pid::Pid, infrastructure::types::Result};

use crate::domain::{
    ledger_entry::{entry::LedgerEntry, entry_type::LedgerEntryType},
    stash::{name::StashName, stash::Stash},
};

#[async_trait]
pub trait StashRepository {
    async fn find_by_pid(&self, pid: &Pid) -> Result<Option<Stash>>;
    async fn exists_with_name_for_user(&self, user_id: &Pid, name: &StashName) -> Result<bool>;
    async fn save(&self, stash: &Stash) -> Result<()>;
}

#[derive(Builder, Default)]
#[builder(setter(into))]
pub struct FindManyLedgerQuery {
    pub user_id: Option<Pid>,
    pub entry_type: Option<LedgerEntryType>,
    pub limit: u16,
    pub page: u16,
}

#[async_trait]
pub trait LedgerRepository {
    async fn find_by_pid(&self, pid: &Pid) -> Result<Option<LedgerEntry>>;
    async fn find_many(&self, query: FindManyLedgerQuery) -> Result<Vec<LedgerEntry>>;
    async fn save(&self, entry: &LedgerEntry) -> Result<()>;
}
