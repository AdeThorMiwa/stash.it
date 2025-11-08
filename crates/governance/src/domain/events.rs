use std::sync::Arc;

use chrono::Utc;
use shared::{
    domain::value_objects::{date::Date, pid::Pid},
    infrastructure::messaging::event::DomainEvent,
};

use crate::domain::policy::{action::PolicyAction, rule::PolicyRule};

#[derive(Debug)]
pub struct PolicyCreated {
    policy_id: Pid,
    pub principal_id: Pid,
    created_at: Date,
}

impl PolicyCreated {
    pub fn new(policy_id: &Pid, principal_id: &Pid) -> Arc<Self> {
        Arc::new(Self {
            policy_id: policy_id.to_owned(),
            principal_id: principal_id.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for PolicyCreated {
    fn event_type(&self) -> &str {
        "PolicyCreated"
    }

    fn aggregate_id(&self) -> Pid {
        self.policy_id.clone()
    }

    fn occurred_at(&self) -> Date {
        self.created_at.clone()
    }
}

#[derive(Debug)]
pub struct PolicyRuleAdded {
    policy_id: Pid,
    pub rule: PolicyRule,
    created_at: Date,
}

impl PolicyRuleAdded {
    pub fn new(policy_id: &Pid, rule: &PolicyRule) -> Arc<Self> {
        Arc::new(Self {
            policy_id: policy_id.to_owned(),
            rule: rule.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for PolicyRuleAdded {
    fn event_type(&self) -> &str {
        "PolicyRuleAdded"
    }

    fn aggregate_id(&self) -> Pid {
        self.policy_id.clone()
    }

    fn occurred_at(&self) -> Date {
        self.created_at.clone()
    }
}

#[derive(Debug)]
pub struct PolicyActionAdded {
    policy_id: Pid,
    pub action: PolicyAction,
    created_at: Date,
}

impl PolicyActionAdded {
    pub fn new(policy_id: &Pid, action: &PolicyAction) -> Arc<Self> {
        Arc::new(Self {
            policy_id: policy_id.to_owned(),
            action: action.to_owned(),
            created_at: Utc::now(),
        })
    }
}

impl DomainEvent for PolicyActionAdded {
    fn event_type(&self) -> &str {
        "PolicyActionAdded"
    }

    fn aggregate_id(&self) -> Pid {
        self.policy_id.clone()
    }

    fn occurred_at(&self) -> Date {
        self.created_at.clone()
    }
}
