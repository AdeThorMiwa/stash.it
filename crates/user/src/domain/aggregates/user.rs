use std::sync::Arc;

use crate::domain::value_objects::email::EmailAddress;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use shared::{
    domain::{
        entity::Entity,
        events::user::{UserCreated, UserStatusUpdated},
        value_objects::{date::Date, pid::Pid, user_status::UserStatus},
    },
    infrastructure::messaging::event::DomainEvent,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pid: Pid,
    email: EmailAddress,
    status: UserStatus,
    #[allow(dead_code)]
    created_at: Date,
    last_login_at: Date,
    #[serde(skip)]
    events: Vec<Arc<dyn DomainEvent>>,
}

impl User {
    /// create a new user
    pub fn new(email: EmailAddress) -> Self {
        let pid = Pid::new();

        Self {
            pid: pid.to_owned(),
            email,
            status: UserStatus::PendingProfile,
            created_at: Utc::now(),
            last_login_at: Utc::now(),
            events: Self::get_initial_events(&pid),
        }
    }

    fn get_initial_events(pid: &Pid) -> Vec<Arc<dyn DomainEvent>> {
        let mut events: Vec<Arc<dyn DomainEvent>> = Vec::new();
        events.push(UserCreated::new(&pid));
        events
    }

    /// update user status to `new_status`
    pub fn update_status(&mut self, new_status: &UserStatus) {
        let old_status = self.status.clone();
        self.status = new_status.clone();
        self.events.push(UserStatusUpdated::new(self.get_pid(), &old_status, new_status));
    }

    /// update user last login time to now
    pub fn update_last_login(&mut self) {
        self.last_login_at = Utc::now()
    }
}

/// Getters
impl User {
    pub fn get_pid(&self) -> &Pid {
        &self.pid
    }

    pub fn get_email(&self) -> &EmailAddress {
        &self.email
    }

    pub fn get_status(&self) -> &UserStatus {
        &self.status
    }
}

impl Entity for User {
    fn drain_events(&mut self) -> Vec<Arc<dyn DomainEvent>> {
        self.events.drain(..).collect()
    }
}
