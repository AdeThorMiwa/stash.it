use serde::{Deserialize, Serialize};

use crate::domain::policy::{intent::Intent, trigger::PolicyActionTrigger};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyActionStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "PAUSED")]
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAction {
    name: String,
    triggers: Vec<PolicyActionTrigger>,
    intent: Intent,
    status: PolicyActionStatus,
}
