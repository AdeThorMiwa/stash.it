use chrono::Utc;
use shared::{
    domain::value_objects::{date::Date, pid::Pid},
    infrastructure::messaging::event::DomainEvent,
};

#[derive(Debug)]
pub struct UserCreatedEvent {
    user_id: Pid,
    created_at: Date,
}

impl UserCreatedEvent {
    pub fn new(user_id: Pid) -> Self {
        Self {
            user_id,
            created_at: Utc::now(),
        }
    }
}

impl DomainEvent for UserCreatedEvent {
    fn event_type(&self) -> &str {
        "UserCreated"
    }

    fn aggregate_id(&self) -> Pid {
        self.user_id.clone()
    }

    fn occurred_at(&self) -> Date {
        self.created_at.clone()
    }
}
