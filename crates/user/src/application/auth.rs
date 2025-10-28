use crate::{
    application::{mailing::MailingService, session::SessionManagementService, user::UserManagementService},
    domain::value_objects::email::EmailAddress,
    infrastructure::auth::jwt_service::JWTService,
};
use di::injectable;
use shared::{
    domain::value_objects::pid::Pid,
    infrastructure::types::{
        Result,
        error::{DomainError, Error},
    },
};
use std::sync::Arc;

#[injectable]
pub struct AuthenticationService {
    session_service: Arc<SessionManagementService>,
    user_service: Arc<UserManagementService>,
    mail_service: Arc<MailingService>,
    jwt_service: Arc<JWTService>,
}

// @todo logout method
impl AuthenticationService {
    pub async fn create_new_session(&self, email: &EmailAddress) -> Result<Pid> {
        let user = match self.user_service.get_user_by_email(&email).await? {
            Some(user) => user,
            None => self.user_service.create_user(&email).await?,
        };
        self.session_service.expire_unused_session(&user.get_pid()).await?;
        let session = self.session_service.create_session(&user.get_pid()).await?;
        self.mail_service.send_authentication_mail(&user, &session).await?;

        Ok(session.get_pid().to_owned())
    }

    pub async fn activate_session(&self, session_id: &Pid, code: &str) -> Result<String> {
        let mut session = match self.session_service.get_session_by_id(session_id).await? {
            Some(session) => session,
            None => return Err(Error::DomainError(DomainError::EntityNotFound)),
        };

        if session.has_expired() || !session.is_valid_code(code) || session.activated() {
            return Err(Error::DomainError(DomainError::EntityInvalid));
        }

        self.session_service.activate_session(&mut session).await?;
        let user = self.user_service.update_user_last_login(session.get_user_id()).await?;
        let token = self.jwt_service.generate_token(&user)?;

        // @todo publish UserAuthenticated
        Ok(token)
    }

    pub async fn is_valid_session(&self, session_id: &Pid) -> Result<bool> {
        if let Some(session) = self.session_service.get_session_by_id(session_id).await? {
            if session.activated() && !session.has_expired() {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub async fn terminate_session(&self, session_id: &Pid) -> Result<()> {
        let mut session = self
            .session_service
            .get_session_by_id(session_id)
            .await?
            .ok_or(Error::DomainError(DomainError::EntityNotFound))?;

        self.session_service.expire_session(&mut session).await?;
        Ok(())
    }
}
