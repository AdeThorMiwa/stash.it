use crate::domain::stash::{name::StashName, status::StashStatus, tag::Tag};
use shared::domain::value_objects::{mula::Mula, pid::Pid};

pub struct CreateStashCommand {
    pub user_id: Pid,
    pub name: StashName,
    pub tags: Vec<Tag>,
}

pub struct GetStashCommand {
    pub stash_id: Pid,
}

pub struct UpdateStashStatusCommand {
    pub stash_id: Pid,
    pub new_status: StashStatus,
}

pub struct UpdateStashBalanceCommand {
    pub stash_id: Pid,
    pub new_balance: Mula,
}
