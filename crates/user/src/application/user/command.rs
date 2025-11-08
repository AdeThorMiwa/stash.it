use shared::domain::value_objects::{pid::Pid, user_status::UserStatus, wallet_address::WalletAddress};

use crate::domain::value_objects::display_name::DisplayName;

pub struct CreateUserProfileCommand {
    pub user_id: Pid,
    pub display_name: DisplayName,
    pub wallet_address: WalletAddress,
}

pub struct UpdateUserStatusCommand {
    pub user_id: Pid,
    pub new_status: UserStatus,
}
