use crate::domain::{aggregates::user::User, value_objects::email::EmailAddress};
use async_trait::async_trait;
use shared::{domain::value_objects::pid::Pid, infrastructure::types::Result};

#[async_trait]
pub trait UserRepository {
    async fn find_by_email(&self, email: &EmailAddress) -> Result<Option<User>>;
    async fn find_by_pid(&self, pid: &Pid) -> Result<Option<User>>;
    async fn save(&self, user: &User) -> Result<()>;
}
