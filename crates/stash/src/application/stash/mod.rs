use crate::{
    application::stash::command::{CreateStashCommand, UpdateStashBalanceCommand, UpdateStashStatusCommand},
    domain::{
        events::{StashBalanceUpdatedEvent, StashCreatedEvent, StashStatusUpdatedEvent},
        repositories::StashRepository,
        stash::stash::Stash,
    },
};
use di::injectable;
use shared::infrastructure::{
    messaging::EventBus,
    types::{
        Result,
        error::{DomainError, Error},
    },
};
use std::sync::Arc;

pub mod command;

#[injectable]
pub struct StashService {
    stash_repo: Arc<dyn StashRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl StashService {
    pub async fn create_stash(&self, command: CreateStashCommand) -> Result<Stash> {
        self.assert_can_create_stash(&command).await?;
        let stash = Stash::new(&command.user_id, &command.name, &command.tags);
        self.stash_repo.save(&stash).await?;
        let stash_created_event = StashCreatedEvent::new(stash.get_pid(), stash.get_user_id());
        self.event_bus.publish(stash_created_event).await?;
        Ok(stash)
    }

    pub async fn update_stash_status(&self, command: UpdateStashStatusCommand) -> Result<()> {
        let mut stash = self
            .stash_repo
            .find_by_pid(&command.stash_id)
            .await?
            .ok_or(Error::DomainError(DomainError::EntityNotFound))?;

        stash.update_status(&command.new_status);
        self.stash_repo.save(&stash).await?;
        let stash_status_updated_event = StashStatusUpdatedEvent::new(stash.get_pid(), stash.get_status());
        self.event_bus.publish(stash_status_updated_event).await?;
        Ok(())
    }

    pub async fn update_stash_balance(&self, command: UpdateStashBalanceCommand) -> Result<()> {
        let mut stash = self
            .stash_repo
            .find_by_pid(&command.stash_id)
            .await?
            .ok_or(Error::DomainError(DomainError::EntityNotFound))?;

        stash.update_balance(&command.asset, &command.new_balance);
        self.stash_repo.save(&stash).await?;
        let stash_balance_updated_event = StashBalanceUpdatedEvent::new(stash.get_pid(), &command.asset, &command.new_balance);
        self.event_bus.publish(stash_balance_updated_event).await?;
        Ok(())
    }

    async fn assert_can_create_stash(&self, command: &CreateStashCommand) -> Result<()> {
        let tag_len = command.tags.len();
        if tag_len > Self::max_tag_len() {
            return Err(Error::AssertError(format!(
                "max tags exceeded. max: {} got: {}",
                Self::max_tag_len(),
                tag_len
            )));
        }

        if self.stash_repo.exists_with_name_for_user(&command.user_id, &command.name).await? {
            return Err(Error::AssertError("stash with name already exist".to_string()));
        }

        Ok(())
    }

    /// maximum tags allowed on a single stash
    fn max_tag_len() -> usize {
        10
    }
}
