use async_trait::async_trait;
use shared::{domain::value_objects::pid::Pid, infrastructure::types::Result};

use crate::domain::stash::{name::StashName, stash::Stash};

#[async_trait]
pub trait StashRepository {
    async fn find_by_pid(&self, pid: &Pid) -> Result<Option<Stash>>;
    async fn exists_with_name_for_user(&self, user_id: &Pid, name: &StashName) -> Result<bool>;
    async fn save(&self, stash: &Stash) -> Result<()>;
}
