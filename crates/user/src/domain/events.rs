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
    pub fn new(user_id: Pid) -> Box<Self> {
        Box::new(Self {
            user_id,
            created_at: Utc::now(),
        })
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

#[derive(Debug)]
pub struct ProfileCreatedEvent {
    user_id: Pid,
    profile_id: Pid,
    created_at: Date,
}

impl ProfileCreatedEvent {
    pub fn new(user_id: Pid, profile_id: Pid) -> Box<Self> {
        Box::new(Self {
            user_id,
            profile_id,
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for ProfileCreatedEvent {
    fn event_type(&self) -> &str {
        "ProfileCreated"
    }

    fn aggregate_id(&self) -> Pid {
        self.user_id.clone()
    }

    fn occurred_at(&self) -> Date {
        self.created_at.clone()
    }
}
