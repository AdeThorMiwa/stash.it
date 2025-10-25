use crate::domain::{
    aggregates::user::User,
    entities::profile::Profile,
    events::{ProfileCreatedEvent, UserCreatedEvent},
    repositories::{ProfileRepository, UserRepository},
    value_objects::{display_name::DisplayName, email::EmailAddress, user_status::UserStatus},
};
use macros::inject;
use shared::{
    domain::value_objects::{pid::Pid, wallet_address::WalletAddress},
    infrastructure::{
        messaging::EventBus,
        types::{
            Result,
            error::{DomainError, Error},
        },
    },
};
use std::sync::Arc;

#[inject]
pub struct UserManagementService {
    user_repo: Arc<dyn UserRepository>,
    profile_repo: Arc<dyn ProfileRepository>,
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
        let user_created_event = UserCreatedEvent::new(user.get_pid());
        self.event_bus.publish(user_created_event).await?;
        Ok(user)
    }

    pub async fn update_user_status(&self, user_id: &Pid, new_status: UserStatus) -> Result<User> {
        if let Some(mut user) = self.user_repo.find_by_pid(user_id).await? {
            user.update_status(new_status);
            self.user_repo.save(&user).await?;
            return Ok(user);
        }

        Err(Error::DomainError(DomainError::EntityNotFound))
    }

    pub async fn update_user_last_login(&self, user_id: &Pid) -> Result<()> {
        if let Some(mut user) = self.user_repo.find_by_pid(user_id).await? {
            user.update_last_login();
            self.user_repo.save(&user).await?;
            return Ok(());
        }

        Err(Error::DomainError(DomainError::EntityNotFound))
    }

    pub async fn create_user_profile(
        &self,
        user_id: &Pid,
        display_name: &DisplayName,
        wallet_address: &WalletAddress,
    ) -> Result<Profile> {
        let user = self
            .user_repo
            .find_by_pid(&user_id)
            .await?
            .ok_or(Error::DomainError(DomainError::EntityNotFound))?;

        if let Some(_) = self.profile_repo.find_by_user_id(&user_id).await? {
            return Err(Error::DomainError(DomainError::EntityAlreadyExist));
        }

        let profile = Profile::new(&user.get_pid(), display_name, wallet_address);
        self.profile_repo.save(&profile).await?;
        let event = ProfileCreatedEvent::new(user_id.clone(), profile.get_pid());
        self.event_bus.publish(event).await?;

        Ok(profile)
    }
}
