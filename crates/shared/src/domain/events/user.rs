use std::sync::Arc;

use chrono::Utc;

use crate::{
    domain::value_objects::{date::Date, pid::Pid, user_status::UserStatus},
    infrastructure::messaging::event::DomainEvent,
};

#[derive(Debug)]
pub struct UserCreated {
    user_id: Pid,
    created_at: Date,
}

impl UserCreated {
    pub fn new(user_id: &Pid) -> Arc<Self> {
        Arc::new(Self {
            user_id: user_id.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for UserCreated {
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
pub struct ProfileCreated {
    user_id: Pid,
    #[allow(dead_code)]
    profile_id: Pid,
    created_at: Date,
}

impl ProfileCreated {
    pub fn new(user_id: &Pid, profile_id: &Pid) -> Arc<Self> {
        Arc::new(Self {
            user_id: user_id.to_owned(),
            profile_id: profile_id.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for ProfileCreated {
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
pub struct SessionActivated {
    user_id: Pid,
    pub session_id: Pid,
    created_at: Date,
}

impl SessionActivated {
    pub fn new(user_id: &Pid, session_id: &Pid) -> Arc<Self> {
        Arc::new(Self {
            user_id: user_id.to_owned(),
            session_id: session_id.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for SessionActivated {
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
pub struct SessionTerminated {
    user_id: Pid,
    pub session_id: Pid,
    created_at: Date,
}

impl SessionTerminated {
    pub fn new(user_id: &Pid, session_id: &Pid) -> Arc<Self> {
        Arc::new(Self {
            user_id: user_id.to_owned(),
            session_id: session_id.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for SessionTerminated {
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

#[derive(Debug)]
pub struct UserStatusUpdated {
    pub user_id: Pid,
    pub old_status: UserStatus,
    pub new_status: UserStatus,
    pub created_at: Date,
}

impl UserStatusUpdated {
    pub fn new(user_id: &Pid, old_status: &UserStatus, new_status: &UserStatus) -> Arc<Self> {
        Arc::new(Self {
            user_id: user_id.to_owned(),
            old_status: old_status.to_owned(),
            new_status: new_status.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for UserStatusUpdated {
    fn event_type(&self) -> &str {
        "UserStatusUpdated"
    }

    fn aggregate_id(&self) -> Pid {
        self.user_id.clone()
    }

    fn occurred_at(&self) -> Date {
        self.created_at.clone()
    }
}
