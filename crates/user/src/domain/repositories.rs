use crate::domain::{
    aggregates::user::User,
    entities::{profile::Profile, session::Session},
    value_objects::email::EmailAddress,
};
use async_trait::async_trait;
use shared::{domain::value_objects::pid::Pid, infrastructure::types::Result};

#[async_trait]
pub trait UserRepository {
    async fn find_by_email(&self, email: &EmailAddress) -> Result<Option<User>>;
    async fn find_by_pid(&self, pid: &Pid) -> Result<Option<User>>;
    async fn save(&self, user: &User) -> Result<()>;
}

#[async_trait]
pub trait SessionRepository {
    async fn find_by_pid(&self, pid: &Pid) -> Result<Option<Session>>;
    async fn save(&self, session: &Session) -> Result<()>;
    async fn expire_unused(&self, user_id: &Pid) -> Result<()>;
}

#[async_trait]
pub trait ProfileRepository {
    async fn find_by_user_id(&self, pid: &Pid) -> Result<Option<Profile>>;
    async fn save(&self, profile: &Profile) -> Result<()>;
}
