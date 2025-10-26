use crate::infrastructure::{
    messaging::{EventBus, EventHandler, event::DomainEvent},
    types::Result,
};
use async_trait::async_trait;
use di::injectable;
use std::sync::Arc;

#[injectable(EventBus)]
pub struct InMemoryEventBus {}

#[async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<()> {
        Ok(())
    }

    async fn subscribe(&self, event_type: &str, handler: Arc<dyn EventHandler>) -> Result<()> {
        Ok(())
    }
}
