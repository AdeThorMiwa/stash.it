use crate::domain::policy::policy::Policy;
use async_trait::async_trait;
use shared::{domain::value_objects::pid::Pid, infrastructure::types::Result};

#[async_trait]
pub trait PolicyRepository: Sync + Send {
    async fn find_by_principal_id(&self, principal_id: &Pid) -> Result<Option<Policy>>;
    async fn save(&self, policy: &Policy) -> Result<()>;
}
