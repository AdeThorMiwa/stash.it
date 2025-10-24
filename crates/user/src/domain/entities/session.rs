use crate::domain::value_objects::otp_code::OtpCode;
use chrono::{TimeDelta, Utc};
use shared::domain::value_objects::{date::Date, pid::Pid};

pub struct Session {
    pub(crate) pid: Pid,
    pub(crate) user_id: Pid,
    pub(crate) code: OtpCode,
    pub(crate) expires_at: Date,
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

    fn expiry() -> Date {
        Utc::now() + TimeDelta::minutes(10)
    }
}
