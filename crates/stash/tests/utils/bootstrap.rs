use di::{Injectable, ServiceCollection, ServiceProvider};
use shared::infrastructure::messaging::memory::InMemoryEventBus;
use stash::application::{ledger::LedgerService, stash::StashService};

use crate::utils::repositories::{StubLedgerRepository, StubStashRepository};

pub fn bootstrap() -> ServiceProvider {
    ServiceCollection::new()
        .add(StashService::singleton())
        .add(LedgerService::singleton())
        .add(StubStashRepository::singleton())
        .add(StubLedgerRepository::singleton())
        .add(InMemoryEventBus::singleton())
        .build_provider()
        .unwrap()
}
