use crate::domain::{entities::profile::Profile, repositories::ProfileRepository};
use async_trait::async_trait;
use di::injectable;
use shared::{domain::value_objects::pid::Pid, infrastructure::types::Result};
use tokio::sync::Mutex;

#[injectable(ProfileRepository)]
#[derive(Default)]
pub struct PostgresProfileRepository {
    profiles: Mutex<Vec<Profile>>,
}

#[async_trait]
impl ProfileRepository for PostgresProfileRepository {
    async fn find_by_user_id(&self, pid: &Pid) -> Result<Option<Profile>> {
        let profiles = self.profiles.lock().await;
        let profile = profiles.iter().find(|u| u.get_user_id() == pid).map(|u| u.clone());
        Ok(profile)
    }

    async fn save(&self, profile: &Profile) -> Result<()> {
        let mut profiles = self.profiles.lock().await;
        profiles.retain(|u| u.get_pid() != profile.get_pid());
        profiles.push(profile.clone());
        Ok(())
    }
}
