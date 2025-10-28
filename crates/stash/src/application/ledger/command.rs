use crate::domain::ledger_entry::entry_type::LedgerEntryType;
use shared::domain::value_objects::{asset::Asset, mula::Mula, pid::Pid};

pub struct CreateLedgerEntryCommand {
    pub stash_id: Pid,
    pub entry_type: LedgerEntryType,
    pub amount: Mula,
    pub asset: Asset,
    pub upstream_ref_id: Pid,
}

pub struct GetLedgerEntryCommand {
    pub entry_id: Pid,
}

pub struct GetLedgerEntriesCommand {
    pub user_id: Option<Pid>,
    pub page: u16,
    pub limit: Option<u16>,
}
