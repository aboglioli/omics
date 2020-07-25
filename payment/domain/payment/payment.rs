use common::error::Error;
use common::model::{AggregateRoot, StatusHistory};

use crate::domain::payment::{Amount, PaymentStatus};

pub type PaymentID = String;

pub struct Payment {
    base: AggregateRoot<PaymentID>,
    amount: Amount,
    status: StatusHistory<PaymentStatus, String>,
}

impl Payment {
    pub fn new(id: PaymentID, amount: Amount) -> Result<Payment, Error> {
        Ok(Payment {
            base: AggregateRoot::new(id),
            amount,
            status: StatusHistory::init(PaymentStatus::Pending),
        })
    }
}
