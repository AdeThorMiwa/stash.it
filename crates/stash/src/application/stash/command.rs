use shared::domain::value_objects::{asset::Asset, mula::Mula, pid::Pid};

use crate::domain::stash::{name::StashName, status::StashStatus, tag::Tag};

pub struct CreateStashCommand {
    pub user_id: Pid,
    pub name: StashName,
    pub tags: Vec<Tag>,
}

pub struct UpdateStashStatusCommand {
    pub stash_id: Pid,
    pub new_status: StashStatus,
}

pub struct UpdateStashBalanceCommand {
    pub stash_id: Pid,
    pub asset: Asset,
    pub new_balance: Mula,
}
