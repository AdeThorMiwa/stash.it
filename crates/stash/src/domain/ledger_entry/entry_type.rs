use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LedgerEntryType {
    DEBIT,
    CREDIT,
}
