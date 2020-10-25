use async_trait::async_trait;

use common::config::Config;
use common::result::Result;
use identity::domain::user::User;

use crate::domain::payment::PaymentService;

pub struct DevelopmentPaymentService;

impl DevelopmentPaymentService {
    pub fn new() -> Self {
        DevelopmentPaymentService
    }
}

#[async_trait]
impl PaymentService for DevelopmentPaymentService {
    async fn get_payment_link(
        &self,
        _title: String,
        _description: String,
        _unit_price: f64,
        external_reference: String,
        _payer: &User,
    ) -> Result<String> {
        let config = Config::get();

        Ok(format!(
            "{}/webhook/development?reference={}",
            config.api_url(),
            external_reference,
        ))
    }

    async fn get_external_reference_from_payment(&self, payment_id: String) -> Result<String> {
        Ok(payment_id)
    }
}
