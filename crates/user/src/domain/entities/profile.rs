use std::sync::Arc;

use crate::domain::value_objects::display_name::DisplayName;
use serde::{Deserialize, Serialize};
use shared::{
    domain::{
        entity::Entity,
        events::user::ProfileCreated,
        value_objects::{pid::Pid, wallet_address::WalletAddress},
    },
    infrastructure::messaging::event::DomainEvent,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pid: Pid,
    user_id: Pid,
    display_name: DisplayName,
    wallet_address: WalletAddress,
    #[serde(skip)]
    events: Vec<Arc<dyn DomainEvent>>,
}

impl Profile {
    pub fn new(user_id: &Pid, display_name: &DisplayName, wallet_address: &WalletAddress) -> Self {
        let pid = Pid::new();

        Self {
            pid: pid.to_owned(),
            user_id: user_id.to_owned(),
            display_name: display_name.clone(),
            wallet_address: wallet_address.clone(),
            events: Self::get_initial_events(&pid, user_id),
        }
    }

    fn get_initial_events(pid: &Pid, user_id: &Pid) -> Vec<Arc<dyn DomainEvent>> {
        let mut events: Vec<Arc<dyn DomainEvent>> = Vec::new();
        events.push(ProfileCreated::new(user_id, pid));
        events
    }

    pub fn get_pid(&self) -> &Pid {
        &self.pid
    }

    pub fn get_user_id(&self) -> &Pid {
        &self.user_id
    }
}

impl Entity for Profile {
    fn drain_events(&mut self) -> Vec<Arc<dyn DomainEvent>> {
        self.events.drain(..).collect()
    }
}
