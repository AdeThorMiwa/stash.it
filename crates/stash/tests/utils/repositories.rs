use async_trait::async_trait;
use di::injectable;
use shared::{domain::value_objects::pid::Pid, infrastructure::types::Result};
use stash::domain::{
    ledger_entry::entry::LedgerEntry,
    repositories::{FindManyLedgerQuery, FindManyStashQuery, LedgerRepository, StashRepository},
    stash::{name::StashName, stash::Stash},
};
use tokio::sync::Mutex;

#[injectable(StashRepository)]
pub struct StubStashRepository {
    stashes: Mutex<Vec<Stash>>,
}

#[async_trait]
impl StashRepository for StubStashRepository {
    async fn find_by_pid(&self, pid: &Pid) -> Result<Option<Stash>> {
        let stashes = self.stashes.lock().await;
        let stash = stashes.iter().find(|s| s.get_pid() == pid).map(|s| s.clone());
        Ok(stash)
    }

    async fn find_many(&self, _query: FindManyStashQuery) -> Result<Vec<Stash>> {
        let stashes = self.stashes.lock().await;
        Ok(stashes.clone())
    }

    async fn exists_with_name_for_user(&self, user_id: &Pid, name: &StashName) -> Result<bool> {
        let stashes = self.stashes.lock().await;
        let stash = stashes.iter().find(|s| s.get_user_id() == user_id && s.get_name() == name);
        Ok(stash.is_some())
    }

    async fn save(&self, stash: &Stash) -> Result<()> {
        let mut stashs = self.stashes.lock().await;
        stashs.retain(|s| s.get_pid() != stash.get_pid());
        stashs.push(stash.clone());
        Ok(())
    }
}

#[injectable(LedgerRepository)]
pub struct StubLedgerRepository {
    entries: Mutex<Vec<LedgerEntry>>,
}

#[async_trait]
impl LedgerRepository for StubLedgerRepository {
    async fn find_by_pid(&self, pid: &Pid) -> Result<Option<LedgerEntry>> {
        let entries = self.entries.lock().await;
        let entry = entries.iter().find(|e| e.get_pid() == pid).map(|s| s.clone());
        Ok(entry)
    }

    async fn find_many(&self, query: FindManyLedgerQuery) -> Result<Vec<LedgerEntry>> {
        println!("{:?}", query);
        let entries = self.entries.lock().await;
        Ok(entries.clone())
    }

    async fn save(&self, entry: &LedgerEntry) -> Result<()> {
        let mut entries = self.entries.lock().await;
        entries.retain(|e| e.get_pid() != entry.get_pid());
        entries.push(entry.clone());
        Ok(())
    }
}
