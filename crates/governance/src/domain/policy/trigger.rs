use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyActionTrigger {
    Temporal {},
    StateChange {},
    Event {},
    Manual,
}
