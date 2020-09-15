use async_trait::async_trait;

use common::result::Result;

#[async_trait]
pub trait EmailService {
    async fn send(&self, email: &Email) -> Result<()>;
}
