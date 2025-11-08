use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "params")]
pub enum Intent {
    #[serde(rename = "DEPOSIT")]
    Deposit { from: String, to: String, amount: u32 },
    #[serde(rename = "WITHDRAWAL")]
    Withrawal { from: String, to: String, amount: u32 },
    #[serde(rename = "CUSTOM")]
    Custom { intent_type: String },
}
