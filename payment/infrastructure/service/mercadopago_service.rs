use async_trait::async_trait;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use serde_json::json;

use common::config::Config;
use common::error::Error;
use common::result::Result;
use identity::domain::user::User;

use crate::domain::payment::PaymentService;

#[derive(Deserialize)]
pub struct MPPreferenceResponse {
    init_point: String,
}

#[derive(Deserialize)]
pub struct MPPayment {
    external_reference: String,
}

pub struct MercadoPagoService {
    public_key: String,
    access_token: String,
}

impl MercadoPagoService {
    pub fn new() -> Self {
        let config = Config::get();

        MercadoPagoService {
            public_key: config.mp_public_key().to_string(),
            access_token: config.mp_access_token().to_string(),
        }
    }
}

#[async_trait]
impl PaymentService for MercadoPagoService {
    async fn get_payment_link(
        &self,
        title: String,
        description: String,
        unit_price: f64,
        external_reference: String,
        payer: &User,
    ) -> Result<String> {
        let body = json!({
            "items": [{
                "title": title,
                "description": description,
                "quantity": 1,
                "currency_id": "ARS",
                "unit_price": unit_price,
            }],
            "statement_descriptor": "OMICS",
            "payment_methods": {
                "installments": 1,
                "default_installments": 1
            },
            "payer": {
                "email": payer.identity().email().to_string(),
            },
            "external_reference": external_reference,
        });

        let client = reqwest::Client::new();
        let res = client
            .post("https://api.mercadopago.com/checkout/preferences")
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .json(&body)
            .send()
            .await
            .map_err(|err| Error::new("mercado_pago_service", "get_payment_link").wrap_raw(err))?;

        let res: MPPreferenceResponse = res
            .json()
            .await
            .map_err(|err| Error::new("response", "deserialize").wrap_raw(err))?;

        Ok(res.init_point)
    }

    async fn get_external_reference_from_payment(&self, payment_id: String) -> Result<String> {
        let client = reqwest::Client::new();
        let res = client
            .get(&format!(
                "https://api.mercadopago.com/v1/payments/{}",
                payment_id
            ))
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .send()
            .await
            .map_err(|err| {
                Error::new(
                    "mercado_pago_service",
                    "get_external_reference_from_payment",
                )
                .wrap_raw(err)
            })?;

        let res: MPPayment = res
            .json()
            .await
            .map_err(|err| Error::new("response", "deserialize").wrap_raw(err))?;

        Ok(res.external_reference)
    }
}
