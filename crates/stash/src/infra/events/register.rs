use std::sync::Arc;

use di::injectable;
use shared::infrastructure::messaging::{EventBus, EventHandler};

#[injectable]
pub struct EventSubscriber {
    event_bus: Arc<dyn EventBus>,
    event_listeners: Vec<Arc<dyn EventHandler>>,
}

impl EventSubscriber {
    pub async fn subscribe_listeners(&self) {
        for listener in &self.event_listeners {
            if let Err(e) = self.event_bus.subscribe(Arc::clone(listener)).await {
                println!("failed to subscribe event: {} error: {:?}", listener.event_type(), e)
            }
        }
    }
}
