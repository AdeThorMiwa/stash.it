use crate::domain::{entities::session::Session, repositories::SessionRepository};
use macros::inject;
use shared::{domain::value_objects::pid::Pid, infrastructure::types::Result};
use std::sync::Arc;

#[inject]
pub struct SessionManagementService {
    session_repo: Arc<dyn SessionRepository>,
}

impl SessionManagementService {
    pub async fn create_session(&self, user_id: &Pid) -> Result<Session> {
        let session = Session::new(user_id);
        self.session_repo.save(&session).await?;
        Ok(session)
    }

    pub async fn expire_unused_session(&self, user_id: &Pid) -> Result<()> {
        self.session_repo.expire_unused(user_id).await
    }

    pub async fn get_session_by_id(&self, session_id: &Pid) -> Result<Option<Session>> {
        self.session_repo.find_by_pid(session_id).await
    }
}
