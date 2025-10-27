use shared::domain::value_objects::{pid::Pid, wallet_address::WalletAddress};

use crate::domain::value_objects::display_name::DisplayName;

pub struct CreateUserProfileCommand {
    pub user_id: Pid,
    pub display_name: DisplayName,
    pub wallet_address: WalletAddress,
}
