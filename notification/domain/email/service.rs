use async_trait::async_trait;

use common::result::Result;

use crate::domain::email::Email;

#[async_trait]
pub trait EmailService: Sync + Send {
    async fn send(&self, email: &Email) -> Result<()>;
}
