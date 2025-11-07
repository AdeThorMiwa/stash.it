use std::str::FromStr;

use di::ServiceProvider;
use shared::{domain::value_objects::pid::Pid, infrastructure::types::Result};
use stash::{
    application::stash::{StashService, command::CreateStashCommand},
    domain::stash::{name::StashName, stash::Stash, tag::Tag},
};

#[allow(dead_code)]
pub async fn prepare_stash(provider: &ServiceProvider) -> Result<Stash> {
    let stash_service = provider.get_required::<StashService>();
    let command = CreateStashCommand {
        name: StashName::from_str("General").unwrap(),
        user_id: Pid::new(),
        tags: vec![Tag::from_str("personal").unwrap()],
    };

    // Act
    stash_service.create_stash(command).await
}
