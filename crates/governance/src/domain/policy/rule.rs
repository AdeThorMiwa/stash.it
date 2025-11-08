use crate::domain::policy::{intent::Intent, predicate::Predicate};
use serde::{Deserialize, Serialize};
use shared::domain::value_objects::pid::Pid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    pid: Pid,
    name: String,
    predicate: Predicate,
    permitted_intents: Vec<Intent>,
    penalty_policy_id: Option<Pid>,
}
