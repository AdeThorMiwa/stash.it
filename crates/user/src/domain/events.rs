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
    pub fn new(user_id: &Pid) -> Box<Self> {
        Box::new(Self {
            user_id: user_id.to_owned(),
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
    #[allow(dead_code)]
    profile_id: Pid,
    created_at: Date,
}

impl ProfileCreatedEvent {
    pub fn new(user_id: &Pid, profile_id: &Pid) -> Box<Self> {
        Box::new(Self {
            user_id: user_id.to_owned(),
            profile_id: profile_id.to_owned(),
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

#[derive(Debug)]
pub struct SessionActivatedEvent {
    user_id: Pid,
    session_id: Pid,
    created_at: Date,
}

impl SessionActivatedEvent {
    pub fn new(user_id: &Pid, session_id: &Pid) -> Box<Self> {
        Box::new(Self {
            user_id: user_id.to_owned(),
            session_id: session_id.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for SessionActivatedEvent {
    fn event_type(&self) -> &str {
        "SessionActivated"
    }

    fn aggregate_id(&self) -> Pid {
        self.user_id.clone()
    }

    fn occurred_at(&self) -> Date {
        self.created_at.clone()
    }
}

#[derive(Debug)]
pub struct SessionTerminatedEvent {
    user_id: Pid,
    session_id: Pid,
    created_at: Date,
}

impl SessionTerminatedEvent {
    pub fn new(user_id: &Pid, session_id: &Pid) -> Box<Self> {
        Box::new(Self {
            user_id: user_id.to_owned(),
            session_id: session_id.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for SessionTerminatedEvent {
    fn event_type(&self) -> &str {
        "SessionTerminated"
    }

    fn aggregate_id(&self) -> Pid {
        self.user_id.clone()
    }

    fn occurred_at(&self) -> Date {
        self.created_at.clone()
    }
}
