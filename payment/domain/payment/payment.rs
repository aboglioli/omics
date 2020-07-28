use common::error::Error;
use common::event::BasicEvent;
use common::model::{AggregateRoot, StatusHistory};

use crate::domain::payment::{Amount, PaymentStatus};

pub type PaymentId = String;

pub struct Payment {
    base: AggregateRoot<PaymentId, BasicEvent>,
    amount: Amount,
    status: StatusHistory<PaymentStatus, String>,
}

impl Payment {
    pub fn new(id: PaymentId, amount: Amount) -> Result<Payment, Error> {
        Ok(Payment {
            base: AggregateRoot::new(id),
            amount,
            status: StatusHistory::init(PaymentStatus::Pending),
        })
    }
}
