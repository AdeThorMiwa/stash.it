use crate::{
    application::policy::command::{CreatePolicyCommand, GetPrincipalPolicyCommand},
    domain::{policy::policy::Policy, repositories::PolicyRepository},
};
use di::injectable;
use shared::{
    domain::entity::Entity,
    infrastructure::{messaging::EventBus, types::Result},
};
use std::sync::Arc;

pub mod command;

#[injectable]
pub struct PolicyService {
    policy_repo: Arc<dyn PolicyRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl PolicyService {
    pub async fn create_policy(&self, command: CreatePolicyCommand) -> Result<Policy> {
        let mut policy = Policy::new(&command.principal_id);
        policy.add_rules(&command.rules);
        policy.add_actions(&command.actions);
        self.policy_repo.save(&policy).await?;
        self.event_bus.publish_many(policy.drain_events()).await?;
        Ok(policy)
    }

    pub async fn get_principal_policy(&self, command: GetPrincipalPolicyCommand) -> Result<Option<Policy>> {
        self.policy_repo.find_by_principal_id(&command.principal_id).await
    }
}
