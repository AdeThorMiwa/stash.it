use crate::domain::value_objects::{email::EmailAddress, user_status::UserStatus};
use chrono::Utc;
use shared::domain::value_objects::{date::Date, pid::Pid};

#[derive(Debug, Clone)]
pub struct User {
    pub(crate) pid: Pid,
    pub(crate) email: EmailAddress,
    pub(crate) status: UserStatus,
    pub(crate) created_at: Date,
    pub(crate) last_login_at: Date,
}

impl User {
    /// create a new user
    pub fn new(email: EmailAddress) -> Self {
        let pid = Pid::new();

        Self {
            pid,
            email,
            status: UserStatus::PendingProfile,
            created_at: Utc::now(),
            last_login_at: Utc::now(),
        }
    }

    /// update user status to `new_status`
    pub fn update_status(&mut self, new_status: UserStatus) {
        self.status = new_status;
    }

    /// update user last login time to now
    pub fn update_last_login(&mut self) {
        self.last_login_at = Utc::now()
    }
}
