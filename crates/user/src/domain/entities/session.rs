use std::sync::Arc;

use crate::domain::value_objects::otp_code::OtpCode;
use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use shared::{
    domain::{
        entity::Entity,
        events::user::{SessionActivated, SessionTerminated},
        value_objects::{date::Date, pid::Pid},
    },
    infrastructure::messaging::event::DomainEvent,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pid: Pid,
    user_id: Pid,
    code: OtpCode,
    activated: bool,
    expires_at: Date,
    #[serde(skip)]
    events: Vec<Arc<dyn DomainEvent>>,
}

impl Session {
    pub fn new(user_id: &Pid) -> Self {
        Self {
            pid: Pid::new(),
            user_id: user_id.clone(),
            code: OtpCode::six_digit(),
            activated: false,
            expires_at: Self::expiry(),
            events: Vec::new(),
        }
    }

    pub fn get_pid(&self) -> &Pid {
        &self.pid
    }

    pub fn get_user_id(&self) -> &Pid {
        &self.user_id
    }

    pub fn get_code(&self) -> &OtpCode {
        &self.code
    }

    fn expiry() -> Date {
        Utc::now() + TimeDelta::minutes(10)
    }

    pub fn has_expired(&self) -> bool {
        self.expires_at.le(&Utc::now())
    }

    pub fn activated(&self) -> bool {
        self.activated
    }

    pub fn is_valid_code(&self, code: &str) -> bool {
        self.code.to_string() == code.to_owned()
    }

    pub fn expire(&mut self) {
        self.expires_at = Utc::now() - TimeDelta::minutes(10);
        self.events.push(SessionTerminated::new(self.get_user_id(), self.get_pid()))
    }

    pub fn activate(&mut self) {
        self.activated = true;
        self.events.push(SessionActivated::new(self.get_user_id(), self.get_pid()));
    }
}

impl Entity for Session {
    fn drain_events(&mut self) -> Vec<Arc<dyn DomainEvent>> {
        self.events.drain(..).collect()
    }
}
