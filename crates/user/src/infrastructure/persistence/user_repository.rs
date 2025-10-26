use std::sync::Arc;

use crate::domain::{aggregates::user::User, repositories::UserRepository, value_objects::email::EmailAddress};
use async_trait::async_trait;
use di::injectable;
use shared::{domain::value_objects::pid::Pid, infrastructure::types::Result};
use tokio::sync::Mutex;

#[injectable(UserRepository)]
#[derive(Default)]
pub struct PostgresUserRepository {
    users: Mutex<Vec<User>>,
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_email(&self, email: &EmailAddress) -> Result<Option<User>> {
        let users = self.users.lock().await;
        let user = users.iter().find(|u| u.get_email() == email).map(|u| u.clone());
        Ok(user)
    }

    async fn find_by_pid(&self, pid: &Pid) -> Result<Option<User>> {
        let users = self.users.lock().await;
        let user = users.iter().find(|u| u.get_pid() == pid).map(|u| u.clone());
        Ok(user)
    }

    async fn save(&self, user: &User) -> Result<()> {
        let mut users = self.users.lock().await;
        users.retain(|u| u.get_pid() != user.get_pid());
        users.push(user.clone());
        Ok(())
    }
}
