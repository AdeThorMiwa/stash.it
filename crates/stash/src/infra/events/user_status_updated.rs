use std::sync::Arc;

use async_trait::async_trait;
use di::injectable;
use shared::{
    domain::{events::user::UserStatusUpdatedEvent, value_objects::user_status::UserStatus},
    infrastructure::{
        messaging::{
            EventHandler,
            event::{DomainEvent, downcast_event},
        },
        types::Result,
    },
};

use crate::{
    application::stash::{
        StashService,
        command::{GetStashesCommand, UpdateStashStatusCommand},
    },
    domain::stash::status::StashStatus,
};

#[injectable(EventHandler)]
pub struct OnUserStatusUpdated {
    stash_service: Arc<StashService>,
}

impl OnUserStatusUpdated {
    pub fn resolve_new_status(&self, user_status: &UserStatus) -> StashStatus {
        match user_status {
            UserStatus::Active => StashStatus::ACTIVE,
            UserStatus::Suspended | UserStatus::PendingProfile => StashStatus::PAUSED,
            UserStatus::Deleted => StashStatus::CLOSED,
        }
    }
}

#[async_trait]
impl EventHandler for OnUserStatusUpdated {
    fn event_type(&self) -> &'static str {
        "UserStatusUpdated"
    }

    async fn handle(&self, event: Box<dyn DomainEvent>) -> Result<()> {
        let event = downcast_event::<UserStatusUpdatedEvent>(&event);
        let command = GetStashesCommand {
            user_id: Some(event.user_id.clone()),
            limit: Some(1000), // fetch all stashes
            page: 1,
        };

        let user_stashes = self.stash_service.get_stashes(command).await?;
        let new_status = self.resolve_new_status(&event.new_status);

        for stash in user_stashes {
            let command = UpdateStashStatusCommand {
                stash_id: stash.get_pid().to_owned(),
                new_status: new_status.clone(),
            };

            self.stash_service.update_stash_status(command).await?;
        }

        Ok(())
    }
}
