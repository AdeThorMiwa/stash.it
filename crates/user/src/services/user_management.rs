use crate::domain::{
    aggregates::user::User,
    events::UserCreatedEvent,
    repositories::UserRepository,
    value_objects::{email::EmailAddress, user_status::UserStatus},
};
use macros::inject;
use shared::{
    domain::value_objects::pid::Pid,
    infrastructure::{
        messaging::EventBus,
        types::{Result, error::Error},
    },
};
use std::sync::Arc;

#[inject]
pub struct UserManagementService {
    user_repo: Arc<dyn UserRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl UserManagementService {
    pub async fn get_user_by_email(&self, email: &EmailAddress) -> Result<Option<User>> {
        self.user_repo.find_by_email(email).await
    }

    pub async fn get_user_by_pid(&self, user_id: &Pid) -> Result<Option<User>> {
        self.user_repo.find_by_pid(user_id).await
    }

    pub async fn create_user(&self, email: &EmailAddress) -> Result<User> {
        let user = User::new(email.clone());
        self.user_repo.save(&user).await?;
        let user_created_event = Box::new(UserCreatedEvent::new(user.pid.clone()));
        self.event_bus.publish(user_created_event).await?;
        Ok(user)
    }

    pub async fn update_user_status(&self, user_id: &Pid, new_status: UserStatus) -> Result<User> {
        if let Some(mut user) = self.user_repo.find_by_pid(user_id).await? {
            user.update_status(new_status);
            self.user_repo.save(&user).await?;
            return Ok(user);
        }

        Err(Error::DomainEntityNotFound)
    }

    pub async fn update_user_last_login(&self, user_id: &Pid) -> Result<()> {
        if let Some(mut user) = self.user_repo.find_by_pid(user_id).await? {
            user.update_last_login();
            self.user_repo.save(&user).await?;
            return Ok(());
        }

        Err(Error::DomainEntityNotFound)
    }
}
