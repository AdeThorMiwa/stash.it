use std::sync::Arc;

use crate::domain::{
    events::{PolicyActionAdded, PolicyCreated, PolicyRuleAdded},
    policy::{action::PolicyAction, rule::PolicyRule},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use shared::{
    domain::{
        entity::Entity,
        value_objects::{date::Date, pid::Pid},
    },
    infrastructure::messaging::event::DomainEvent,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pid: Pid,
    principal_id: Pid,
    rules: Vec<PolicyRule>,
    actions: Vec<PolicyAction>,
    created_at: Date,
    updated_at: Date,
    #[serde(skip)]
    events: Vec<Arc<dyn DomainEvent>>,
}

impl Policy {
    pub fn new(principal_id: &Pid) -> Self {
        let pid = Pid::new();
        Self {
            pid: pid.to_owned(),
            principal_id: principal_id.to_owned(),
            rules: Vec::new(),
            actions: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            events: Self::get_initial_events(&pid, principal_id),
        }
    }

    fn get_initial_events(pid: &Pid, principal_id: &Pid) -> Vec<Arc<dyn DomainEvent>> {
        let mut events: Vec<Arc<dyn DomainEvent>> = Vec::new();
        events.push(PolicyCreated::new(&pid, principal_id));
        events
    }

    pub fn get_pid(&self) -> &Pid {
        &self.pid
    }

    pub fn get_principal(&self) -> &Pid {
        &self.principal_id
    }

    pub fn add_rules(&mut self, rules: &Vec<PolicyRule>) {
        for rule in rules {
            self.add_rule(rule);
        }
    }

    pub fn add_rule(&mut self, rule: &PolicyRule) {
        // @todo validations here
        self.rules.push(rule.to_owned());
        self.events.push(PolicyRuleAdded::new(self.get_pid(), rule));
    }

    pub fn add_actions(&mut self, actions: &Vec<PolicyAction>) {
        for action in actions {
            self.add_action(action);
        }
    }

    pub fn add_action(&mut self, action: &PolicyAction) {
        // @todo validations here
        self.actions.push(action.to_owned());
        self.events.push(PolicyActionAdded::new(self.get_pid(), action));
    }
}

impl Entity for Policy {
    fn drain_events(&mut self) -> Vec<Arc<dyn DomainEvent>> {
        self.events.drain(..).collect()
    }
}
