use crate::domain::{aggregates::user::User, entities::session::Session};
use macros::inject;
use shared::infrastructure::{
    mailing::{Email, Mailer},
    types::Result,
};
use std::sync::Arc;

#[inject]
pub struct MailingService {
    mailer: Arc<dyn Mailer>,
}

impl MailingService {
    pub async fn send_authentication_mail(&self, user: &User, session: &Session) -> Result<()> {
        let email = Email {
            from: "".to_string(),
            to: user.get_email().to_string(),
            subject: "OTP Request".to_owned(),
            html: format!("You OTP code is {}", session.get_code().to_string()),
            ..Default::default()
        };

        self.mailer.mail(&email).await?;
        Ok(())
    }
}
