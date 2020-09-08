use chrono::{DateTime, Duration, Utc};

use common::error::Error;
use common::result::Result;

use crate::domain::payment::{Payment, PaymentId, Status};

#[derive(Debug, Clone)]
pub struct SubscriptionPayment {
    payment_id: PaymentId,
    amount: f64,
    date: DateTime<Utc>,
}

impl SubscriptionPayment {
    pub fn new(payment: Payment) -> Result<Self> {
        if !matches!(payment.status_history().current(), Status::Paid) {
            return Err(Error::new("payment", "not_paid"));
        }

        Ok(SubscriptionPayment {
            payment_id: payment.base().id().clone(),
            amount: payment.amount().value(),
            date: payment.status_history().current_item().date().clone(),
        })
    }

    pub fn payment_id(&self) -> &PaymentId {
        &self.payment_id
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn is_current(&self) -> bool {
        self.date + Duration::days(30) > Utc::now()
    }
}
