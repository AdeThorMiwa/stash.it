use crate::domain::value_objects::display_name::DisplayName;
use shared::domain::value_objects::{pid::Pid, wallet_address::WalletAddress};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Profile {
    pid: Pid,
    user_id: Pid,
    display_name: DisplayName,
    wallet_address: WalletAddress,
}

impl Profile {
    pub fn new(user_id: &Pid, display_name: &DisplayName, wallet_address: &WalletAddress) -> Self {
        Self {
            pid: Pid::new(),
            user_id: user_id.clone(),
            display_name: display_name.clone(),
            wallet_address: wallet_address.clone(),
        }
    }

    pub fn get_pid(&self) -> &Pid {
        &self.pid
    }
}
