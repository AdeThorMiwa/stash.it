use crate::domain::ledger_entry::entry_type::LedgerEntryType;
use shared::domain::value_objects::{mula::Mula, pid::Pid};

pub struct WriteLedgerEntryCommand {
    pub stash_id: Pid,
    pub entry_type: LedgerEntryType,
    pub amount: Mula,
    pub upstream_ref_id: Pid,
}

pub struct ReadLedgerEntryCommand {
    pub entry_id: Pid,
}

pub struct ReadLedgerEntriesCommand {
    pub user_id: Option<Pid>,
    pub page: u16,
    pub limit: Option<u16>,
}
