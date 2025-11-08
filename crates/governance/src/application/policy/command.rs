use shared::domain::value_objects::pid::Pid;

use crate::domain::policy::{action::PolicyAction, rule::PolicyRule};

pub struct CreatePolicyCommand {
    pub principal_id: Pid,
    pub rules: Vec<PolicyRule>,
    pub actions: Vec<PolicyAction>,
}

pub struct GetPrincipalPolicyCommand {
    pub principal_id: Pid,
}
