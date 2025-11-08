use crate::{
    application::ledger::command::{ReadLedgerEntriesCommand, ReadLedgerEntryCommand, WriteLedgerEntryCommand},
    domain::{
        ledger_entry::entry::LedgerEntry,
        repositories::{FindManyLedgerQueryBuilder, LedgerRepository},
    },
};
use di::injectable;
use shared::{
    domain::entity::Entity,
    infrastructure::{
        messaging::EventBus,
        types::{Result, error::Error},
    },
};
use std::sync::Arc;

pub mod command;

#[injectable]
pub struct LedgerService {
    ledger_repo: Arc<dyn LedgerRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl LedgerService {
    pub async fn write_ledger_entry(&self, command: WriteLedgerEntryCommand) -> Result<LedgerEntry> {
        let mut entry = LedgerEntry::new(&command.stash_id, &command.entry_type, &command.amount, &command.upstream_ref_id);
        self.ledger_repo.save(&entry).await?;
        self.event_bus.publish_many(entry.drain_events()).await?;
        Ok(entry)
    }

    pub async fn read_ledger_entries(&self, command: ReadLedgerEntriesCommand) -> Result<Vec<LedgerEntry>> {
        let query = FindManyLedgerQueryBuilder::default()
            .user_id(command.user_id)
            .entry_type(command.entry_type)
            .limit(command.limit.unwrap_or(20))
            .page(command.page)
            .build()
            .map_err(|e| Error::BuilderError(e.to_string()))?;

        Ok(self.ledger_repo.find_many(query).await?)
    }

    pub async fn read_ledger_entry(&self, command: ReadLedgerEntryCommand) -> Result<Option<LedgerEntry>> {
        self.ledger_repo.find_by_pid(&command.entry_id).await
    }
}
