use std::sync::Arc;

use crate::infrastructure::{messaging::event::DomainEvent, types::Result};
use async_trait::async_trait;
pub mod event;

/// Trait for handling domain events
#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: &dyn DomainEvent) -> Result<()>;
}

/// Trait for publishing and subscribing to domain events
#[async_trait]
pub trait EventBus: Sync + Send {
    /// Publishes a domain event to all subscribers
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<()>;

    /// Subscribes a handler to a specific event type
    async fn subscribe(&self, event_type: &str, handler: Arc<dyn EventHandler>) -> Result<()>;
}
