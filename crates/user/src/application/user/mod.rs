use crate::{
    application::user::command::{CreateUserProfileCommand, UpdateUserStatusCommand},
    domain::{
        aggregates::user::User,
        entities::profile::Profile,
        repositories::{ProfileRepository, UserRepository},
        value_objects::email::EmailAddress,
    },
};
use di::injectable;
use shared::{
    domain::{
        events::user::{ProfileCreatedEvent, UserCreatedEvent, UserStatusUpdatedEvent},
        value_objects::pid::Pid,
    },
    infrastructure::{
        messaging::EventBus,
        types::{
            Result,
            error::{DomainError, Error},
        },
    },
};
use std::sync::Arc;

pub mod command;

#[injectable]
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

    pub(crate) async fn create_user(&self, email: &EmailAddress) -> Result<User> {
        let user = User::new(email.clone());
        self.user_repo.save(&user).await?;
        let user_created_event = UserCreatedEvent::new(user.get_pid());
        self.event_bus.publish(user_created_event).await?;
        Ok(user)
    }

    // @todo make pub(crate)
    pub async fn update_user_status(&self, command: UpdateUserStatusCommand) -> Result<User> {
        let mut user = self
            .user_repo
            .find_by_pid(&command.user_id)
            .await?
            .ok_or(Error::DomainError(DomainError::EntityNotFound))?;

        let old_status = user.get_status().clone();
        user.update_status(&command.new_status);
        self.user_repo.save(&user).await?;

        let user_status_updated_event = UserStatusUpdatedEvent::new(user.get_pid(), &old_status, user.get_status());
        self.event_bus.publish(user_status_updated_event).await?;
        return Ok(user);
    }

    pub(crate) async fn update_user_last_login(&self, user_id: &Pid) -> Result<User> {
        if let Some(mut user) = self.user_repo.find_by_pid(user_id).await? {
            user.update_last_login();
            self.user_repo.save(&user).await?;
            return Ok(user);
        }

        Err(Error::DomainError(DomainError::EntityNotFound))
    }

    pub async fn create_user_profile(&self, command: CreateUserProfileCommand) -> Result<Profile> {
        let user = self
            .user_repo
            .find_by_pid(&command.user_id)
            .await?
            .ok_or(Error::DomainError(DomainError::EntityNotFound))?;

        if let Some(_) = self.profile_repo.find_by_user_id(&command.user_id).await? {
            return Err(Error::DomainError(DomainError::EntityAlreadyExist));
        }

        let profile = Profile::new(&user.get_pid(), &command.display_name, &command.wallet_address);
        self.profile_repo.save(&profile).await?;
        let event = ProfileCreatedEvent::new(&command.user_id, profile.get_pid());
        self.event_bus.publish(event).await?;

        Ok(profile)
    }
}
