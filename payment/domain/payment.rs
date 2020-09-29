mod amount;
mod status;
pub use amount::*;
pub use status::*;

use chrono::{Duration, Utc};

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

    pub fn is_current(&self) -> bool {
        let status_item = self.status_history.current_item();

        matches!(status_item.status(), Status::Paid)
            && status_item.date() + Duration::days(30) > Utc::now()
    }

    pub fn pay(&mut self) -> Result<()> {
        let status = self.status_history.current().pay()?;
        self.status_history.add_status(status);
        self.base.update();

        Ok(())
    }

    pub fn reject(&mut self) -> Result<()> {
        let status = self.status_history.current().reject()?;
        self.status_history.add_status(status);
        self.base.update();

        Ok(())
    }

    pub fn cancel(&mut self) -> Result<()> {
        let status = self.status_history.current().cancel()?;
        self.status_history.add_status(status);
        self.base.update();

        Ok(())
    }
}
