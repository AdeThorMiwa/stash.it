use async_trait::async_trait;
use di::injectable;
use shared::{domain::value_objects::pid::Pid, infrastructure::types::Result};
use tokio::sync::Mutex;
use user::domain::{
    aggregates::user::User,
    entities::{profile::Profile, session::Session},
    repositories::{ProfileRepository, SessionRepository, UserRepository},
    value_objects::email::EmailAddress,
};

#[injectable(SessionRepository)]
#[derive(Default)]
pub struct StubSessionRepository {
    sessions: Mutex<Vec<Session>>,
}

#[async_trait]
impl SessionRepository for StubSessionRepository {
    async fn find_by_pid(&self, pid: &Pid) -> Result<Option<Session>> {
        let sessions = self.sessions.lock().await;
        let session = sessions.iter().find(|u| u.get_pid() == pid).map(|u| u.clone());
        Ok(session)
    }

    async fn save(&self, session: &Session) -> Result<()> {
        let mut sessions = self.sessions.lock().await;
        sessions.retain(|u| u.get_pid() != session.get_pid());
        sessions.push(session.clone());
        Ok(())
    }

    async fn expire_unused(&self, user_id: &Pid) -> Result<()> {
        let mut sessions = self.sessions.lock().await;
        sessions.retain(|u| u.get_user_id() != user_id);
        Ok(())
    }
}

#[injectable(ProfileRepository)]
#[derive(Default)]
pub struct StubProfileRepository {
    profiles: Mutex<Vec<Profile>>,
}

#[async_trait]
impl ProfileRepository for StubProfileRepository {
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

#[injectable(UserRepository)]
#[derive(Default)]
pub struct StubUserRepository {
    users: Mutex<Vec<User>>,
}

#[async_trait]
impl UserRepository for StubUserRepository {
    async fn find_by_email(&self, email: &EmailAddress) -> Result<Option<User>> {
        let users = self.users.lock().await;
        let user = users.iter().find(|u| u.get_email() == email).map(|u| u.clone());
        Ok(user)
    }

    async fn find_by_pid(&self, pid: &Pid) -> Result<Option<User>> {
        let users = self.users.lock().await;
        let user = users.iter().find(|u| u.get_pid() == pid).map(|u| u.clone());
        Ok(user)
    }

    async fn save(&self, user: &User) -> Result<()> {
        let mut users = self.users.lock().await;
        users.retain(|u| u.get_pid() != user.get_pid());
        users.push(user.clone());
        Ok(())
    }
}
