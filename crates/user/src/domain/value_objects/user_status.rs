#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UserStatus {
    Active,
    Suspended,
    PendingProfile,
    Deleted,
}
