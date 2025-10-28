use crate::{
    application::ledger::command::{CreateLedgerEntryCommand, GetLedgerEntriesCommand, GetLedgerEntryCommand},
    domain::{
        events::LedgerEntryCreatedEvent,
        ledger_entry::entry::LedgerEntry,
        repositories::{FindManyLedgerQueryBuilder, LedgerRepository},
    },
};
use di::injectable;
use shared::infrastructure::{
    messaging::EventBus,
    types::{Result, error::Error},
};
use std::sync::Arc;

pub mod command;

#[injectable]
pub struct LedgerService {
    ledger_repo: Arc<dyn LedgerRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl LedgerService {
    pub async fn create_ledger_entry(&self, command: CreateLedgerEntryCommand) -> Result<LedgerEntry> {
        let entry = LedgerEntry::new(
            &command.stash_id,
            &command.entry_type,
            &command.amount,
            &command.asset,
            &command.upstream_ref_id,
        );

        self.ledger_repo.save(&entry).await?;
        let ledger_entry_created_event = LedgerEntryCreatedEvent::new(entry.get_stash_id(), entry.get_pid());
        self.event_bus.publish(ledger_entry_created_event).await?;
        Ok(entry)
    }

    pub async fn get_ledger_entries(&self, command: GetLedgerEntriesCommand) -> Result<Vec<LedgerEntry>> {
        let query = FindManyLedgerQueryBuilder::default()
            .user_id(command.user_id)
            .limit(command.limit.unwrap_or(20))
            .page(command.page)
            .build()
            .map_err(|e| Error::BuilderError(e.to_string()))?;

        Ok(self.ledger_repo.find_many(query).await?)
    }

    pub async fn get_ledger_entry(&self, command: GetLedgerEntryCommand) -> Result<Option<LedgerEntry>> {
        self.ledger_repo.find_by_pid(&command.entry_id).await
    }
}
