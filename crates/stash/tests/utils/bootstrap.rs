use di::{Injectable, ServiceCollection, ServiceProvider};
use shared::infrastructure::messaging::memory::InMemoryEventBus;
use stash::{
    application::{ledger::LedgerService, stash::StashService},
    infra::events::{register::EventSubscriber, user_status_updated::OnUserStatusUpdated},
};

use crate::utils::repositories::{StubLedgerRepository, StubStashRepository};

pub async fn bootstrap() -> ServiceProvider {
    let provider = ServiceCollection::new()
        .add(StashService::singleton())
        .add(LedgerService::singleton())
        .add(StubStashRepository::singleton())
        .add(StubLedgerRepository::singleton())
        .add(InMemoryEventBus::singleton())
        .add(EventSubscriber::singleton())
        .add(OnUserStatusUpdated::singleton())
        .build_provider()
        .unwrap();

    let subscriber = provider.get_required::<EventSubscriber>();
    subscriber.subscribe_listeners().await;

    provider
}
