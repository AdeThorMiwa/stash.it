use crate::{
    domain::value_objects::email::EmailAddress,
    services::{mailing::MailingService, session_management::SessionManagementService, user_management::UserManagementService},
};
use macros::inject;
use shared::{domain::value_objects::pid::Pid, infrastructure::types::Result};
use std::sync::Arc;

#[inject]
pub struct AuthenticationService {
    session_service: Arc<SessionManagementService>,
    user_service: Arc<UserManagementService>,
    mail_service: Arc<MailingService>,
}

impl AuthenticationService {
    pub async fn start_authentication(&self, email: &EmailAddress) -> Result<Pid> {
        let user = match self.user_service.get_user_by_email(&email).await? {
            Some(user) => user,
            None => self.user_service.create_user(&email).await?,
        };
        let session = self.session_service.create_session(&user.get_pid()).await?;
        self.mail_service.send_authentication_mail(&user, &session).await?;

        Ok(session.get_pid().to_owned())
    }
}
