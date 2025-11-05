use crate::infrastructure::{
    messaging::{EventBus, EventHandler, event::DomainEvent},
    types::Result,
};
use async_trait::async_trait;
use di::injectable;
use std::sync::Arc;
use tokio::sync::Mutex;

#[injectable(EventBus)]
pub struct InMemoryEventBus {
    published_events: Mutex<Vec<Box<dyn DomainEvent>>>,
}

#[async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<()> {
        let mut published_events = self.published_events.lock().await;
        published_events.push(event);
        Ok(())
    }

    async fn subscribe(&self, _handler: Arc<dyn EventHandler>) -> Result<()> {
        Ok(())
    }

    async fn published(&self, event: Box<dyn DomainEvent>) -> bool {
        let published_events = self.published_events.lock().await;
        published_events.iter().find(|e| e.event_type() == event.event_type()).is_some()
    }
}
