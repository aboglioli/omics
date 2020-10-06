use async_trait::async_trait;

use common::result::Result;

use crate::domain::payment::{Payment, PaymentService};

pub struct FakePaymentService {}

#[async_trait]
impl PaymentService for FakePaymentService {
    async fn pay(&self, email: &str, payment: &Payment) -> Result<()> {
        println!("Pay to: {}, amount: {}", email, payment.amount().value());
        Ok(())
    }
}
