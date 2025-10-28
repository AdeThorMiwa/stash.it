use crate::domain::{entities::session::Session, repositories::SessionRepository};
use async_trait::async_trait;
use di::injectable;
use shared::{domain::value_objects::pid::Pid, infrastructure::types::Result};
use tokio::sync::Mutex;

#[injectable(SessionRepository)]
#[derive(Default)]
pub struct PostgresSessionRepository {
    sessions: Mutex<Vec<Session>>,
}

#[async_trait]
impl SessionRepository for PostgresSessionRepository {
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
