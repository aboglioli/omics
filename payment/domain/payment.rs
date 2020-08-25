mod amount;
mod status;
pub use amount::*;
pub use status::*;

use chrono::{DateTime, Utc};

use common::event::Event;
use common::model::{AggregateRoot, StatusHistory, StringId};
use common::result::Result;

pub type PaymentId = StringId;

#[derive(Debug, Clone)]
pub struct Payment {
    base: AggregateRoot<PaymentId, Event>,
    amount: Amount,
    date: DateTime<Utc>,
    status_history: StatusHistory<Status>,
}

impl Payment {
    pub fn new(id: PaymentId, amount: Amount) -> Result<Self> {
        Ok(Payment {
            base: AggregateRoot::new(id),
            amount,
            date: Utc::now(),
            status_history: StatusHistory::new(Status::WaitingPayment),
        })
    }

    pub fn base(&self) -> &AggregateRoot<PaymentId, Event> {
        &self.base
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn status_history(&self) -> &StatusHistory<Status> {
        &self.status_history
    }
}
