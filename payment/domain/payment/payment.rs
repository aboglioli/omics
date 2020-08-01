use common::event::BasicEvent;
use common::model::{AggregateRoot, StatusHistory};
use common::result::Result;

use crate::domain::payment::{Amount, PaymentStatus};

pub type PaymentId = String;

pub struct Payment {
    base: AggregateRoot<PaymentId, BasicEvent>,
    amount: Amount,
    status: StatusHistory<PaymentStatus, String>,
}

impl Payment {
    pub fn new(id: PaymentId, amount: Amount) -> Result<Payment> {
        Ok(Payment {
            base: AggregateRoot::new(id),
            amount,
            status: StatusHistory::init(PaymentStatus::Pending),
        })
    }

    pub fn base(&self) -> &AggregateRoot<PaymentId, BasicEvent> {
        &self.base
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn status(&self) -> &StatusHistory<PaymentStatus, String> {
        &self.status
    }
}
