mod amount;
mod status;
pub use amount::*;
pub use status::*;

use common::model::{AggregateRoot, StatusHistory, StringId};
use common::result::Result;

pub type PaymentId = StringId;

#[derive(Debug, Clone)]
pub struct Payment {
    base: AggregateRoot<PaymentId>,
    amount: Amount,
    status_history: StatusHistory<Status>,
}

impl Payment {
    pub fn new(id: PaymentId, amount: Amount) -> Result<Self> {
        Ok(Payment {
            base: AggregateRoot::new(id),
            amount,
            status_history: StatusHistory::new(Status::WaitingPayment),
        })
    }

    pub fn build(
        base: AggregateRoot<PaymentId>,
        amount: Amount,
        status_history: StatusHistory<Status>,
    ) -> Self {
        Payment {
            base,
            amount,
            status_history,
        }
    }

    pub fn base(&self) -> &AggregateRoot<PaymentId> {
        &self.base
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn status_history(&self) -> &StatusHistory<Status> {
        &self.status_history
    }

    pub fn pay(&mut self) -> Result<()> {
        let status = self.status_history.current().pay()?;
        self.status_history.add_status(status);

        Ok(())
    }
}
