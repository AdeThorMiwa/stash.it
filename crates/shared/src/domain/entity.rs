use std::sync::Arc;

use crate::infrastructure::messaging::event::DomainEvent;

pub trait Entity: Send + Sync {
    fn drain_events(&mut self) -> Vec<Arc<dyn DomainEvent>>;
}
