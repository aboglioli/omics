use common::error::Error;
use common::model::{Entity, StatusHistory, ID};

use crate::domain::payment::{Amount, PaymentStatus};

pub type PaymentID = String;

pub struct Payment {
    id: ID<PaymentID>,
    amount: Amount,
    status: StatusHistory<PaymentStatus, String>,
}

impl Payment {
    pub fn new(id: PaymentID, amount: Amount) -> Result<Payment, Error> {
        Ok(Payment {
            id: ID::new(id),
            amount,
            status: StatusHistory::init(PaymentStatus::Pending),
        })
    }
}

impl Entity<PaymentID> for Payment {
    fn id(&self) -> &ID<PaymentID> {
        &self.id
    }
}
