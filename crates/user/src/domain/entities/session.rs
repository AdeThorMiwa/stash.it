use crate::domain::value_objects::otp_code::OtpCode;
use chrono::{TimeDelta, Utc};
use shared::domain::value_objects::{date::Date, pid::Pid};

#[derive(Debug, Clone)]
pub struct Session {
    pid: Pid,
    user_id: Pid,
    code: OtpCode,
    expires_at: Date,
}

impl Session {
    pub fn new(user_id: &Pid) -> Self {
        let pid = Pid::new();
        Self {
            pid,
            user_id: user_id.clone(),
            code: OtpCode::six_digit(),
            expires_at: Self::expiry(),
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

    pub fn is_valid_code(&self, code: &str) -> bool {
        self.code.to_string() == code.to_owned()
    }

    pub fn expire(&self) {
        self.expires_at = Utc::now() - TimeDelta::minutes(10)
    }
}
