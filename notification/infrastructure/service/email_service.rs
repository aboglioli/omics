use async_trait::async_trait;

use crate::domain::email::{Email, EmailService};

pub struct MailchimpService {

}

#[async_trait]
impl EmailService for MailchimpService {
    async fn send(&self, email: &Email) -> Result<()> {
        Ok(())
    }
}
