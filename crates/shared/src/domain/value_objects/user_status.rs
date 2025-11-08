use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Suspended,
    PendingProfile,
    Deleted,
}
