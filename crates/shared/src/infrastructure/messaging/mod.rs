use std::sync::Arc;

use crate::infrastructure::{messaging::event::DomainEvent, types::Result};
use async_trait::async_trait;
pub mod event;
pub mod memory;

/// Trait for handling domain events
#[async_trait]
pub trait EventHandler
where
    Self: Send + Sync,
{
    fn event_type(&self) -> &'static str;
    async fn handle(&self, event: Box<dyn DomainEvent>) -> Result<()>;
}

/// Trait for publishing and subscribing to domain events
#[async_trait]
pub trait EventBus: Sync + Send {
    /// Publishes a domain event to all subscribers
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<()>;

    /// Subscribes a handler to a specific event type
    async fn subscribe(&self, handler: Arc<dyn EventHandler>) -> Result<()>;

    #[cfg(feature = "testing")]
    async fn published(&self, event: Box<dyn DomainEvent>) -> bool;
}
