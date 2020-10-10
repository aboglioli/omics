use async_trait::async_trait;

use common::result::Result;

use crate::domain::payment::Payment;

#[async_trait]
pub trait PaymentService: Sync + Send {
    // TODO: check if payment is made through email or something else
    async fn pay(&self, email: &str, payment: &Payment) -> Result<()>;
}
