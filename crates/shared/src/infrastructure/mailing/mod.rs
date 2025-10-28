use crate::infrastructure::types::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
pub mod stub_mailer;

/// The structure representing an email details.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Email {
    /// Mailbox to `From` header
    pub from: String,
    /// Mailbox to `To` header
    pub to: String,
    /// Mailbox to `ReplyTo` header
    pub reply_to: Option<String>,
    /// Subject header to message
    pub subject: String,
    /// Plain text message
    pub text: String,
    /// HTML template
    pub html: String,
    /// BCC header to message
    pub bcc: Option<String>,
    /// CC header to message
    pub cc: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Deliveries {
    pub count: usize,
    pub messages: Vec<String>,
}

#[async_trait]
pub trait Mailer: Sync + Send {
    async fn mail(&self, email: &Email) -> Result<()>;
    async fn deliveries(&self) -> Deliveries;
}
