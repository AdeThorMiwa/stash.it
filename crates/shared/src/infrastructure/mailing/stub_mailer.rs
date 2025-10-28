use super::Email;
use crate::infrastructure::{
    mailing::{Deliveries, Mailer},
    types::Result,
};
use async_trait::async_trait;
use di::injectable;
use tokio::sync::Mutex;

#[injectable(Mailer)]
#[derive(Default)]
pub struct StubMailer {
    deliveries: Mutex<Deliveries>,
}

#[async_trait]
impl Mailer for StubMailer {
    async fn mail(&self, email: &Email) -> Result<()> {
        let mut deliveries = self.deliveries.lock().await;
        deliveries.count += 1;
        let message = format!("{} \n ----------- \n {}", email.html, email.text);
        deliveries.messages.push(message);
        Ok(())
    }

    async fn deliveries(&self) -> Deliveries {
        self.deliveries.lock().await.clone()
    }
}
