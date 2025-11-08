use crate::{
    application::stash::command::{CreateStashCommand, GetStashCommand, GetStashesCommand, UpdateStashBalanceCommand, UpdateStashStatusCommand},
    domain::{
        repositories::{FindManyStashQueryBuilder, StashRepository},
        stash::stash::Stash,
    },
};
use di::injectable;
use shared::{
    domain::entity::Entity,
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
pub struct StashService {
    stash_repo: Arc<dyn StashRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl StashService {
    pub async fn get_stash(&self, command: GetStashCommand) -> Result<Option<Stash>> {
        self.stash_repo.find_by_pid(&command.stash_id).await
    }

    pub async fn get_stashes(&self, command: GetStashesCommand) -> Result<Vec<Stash>> {
        let query = FindManyStashQueryBuilder::default()
            .owner_id(command.owner_id)
            .limit(command.limit.unwrap_or(20))
            .page(command.page)
            .build()
            .map_err(|e| Error::BuilderError(e.to_string()))?;

        self.stash_repo.find_many(query).await
    }

    pub async fn create_stash(&self, command: CreateStashCommand) -> Result<Stash> {
        self.assert_can_create_stash(&command).await?;
        let mut stash = Stash::new(&command.owner_id, &command.name, &command.tags);
        self.stash_repo.save(&stash).await?;
        self.event_bus.publish_many(stash.drain_events()).await?;
        Ok(stash)
    }

    pub async fn update_stash_status(&self, command: UpdateStashStatusCommand) -> Result<Stash> {
        let mut stash = self
            .stash_repo
            .find_by_pid(&command.stash_id)
            .await?
            .ok_or(Error::DomainError(DomainError::EntityNotFound))?;

        stash.update_status(&command.new_status);
        self.stash_repo.save(&stash).await?;
        self.event_bus.publish_many(stash.drain_events()).await?;
        Ok(stash)
    }

    pub async fn update_stash_balance(&self, command: UpdateStashBalanceCommand) -> Result<Stash> {
        let mut stash = self
            .stash_repo
            .find_by_pid(&command.stash_id)
            .await?
            .ok_or(Error::DomainError(DomainError::EntityNotFound))?;

        stash.update_balance(&command.new_balance);
        self.stash_repo.save(&stash).await?;
        self.event_bus.publish_many(stash.drain_events()).await?;
        Ok(stash)
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

        if self.stash_repo.exists_with_name_for_owner(&command.owner_id, &command.name).await? {
            return Err(Error::AssertError("stash with name already exist for owner".to_string()));
        }

        Ok(())
    }

    /// maximum tags allowed on a single stash
    fn max_tag_len() -> usize {
        10
    }
}
