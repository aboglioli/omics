use async_trait::async_trait;

use common::result::Result;
use identity::domain::user::User;

#[async_trait]
pub trait PaymentService: Sync + Send {
    async fn get_payment_link(
        &self,
        title: String,
        description: String,
        unit_price: f64,
        external_reference: String,
        payer: &User,
    ) -> Result<String>;

    async fn get_external_reference_from_payment(&self, payment_id: String) -> Result<String>;
}
