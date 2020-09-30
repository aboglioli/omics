mod plan;
mod repository;
mod status;
pub use plan::*;
pub use repository::*;
pub use status::*;

use common::error::Error;

use common::model::{AggregateRoot, StatusHistory, StringId};
use common::result::Result;
use identity::domain::user::UserId;

use crate::domain::payment::Payment;
use crate::domain::plan::Plan;

pub type SubscriptionId = StringId;

#[derive(Debug, Clone)]
pub struct Subscription {
    base: AggregateRoot<SubscriptionId>,
    user_id: UserId,
    plan: SubscriptionPlan,
    payments: Vec<Payment>,
    status_history: StatusHistory<Status>,
}

impl Subscription {
    pub fn new(id: SubscriptionId, user_id: UserId, plan: Plan) -> Result<Self> {
        let plan = SubscriptionPlan::new(plan)?;

        Ok(Subscription {
            base: AggregateRoot::new(id),
            user_id,
            plan,
            payments: Vec::new(),
            status_history: StatusHistory::new(Status::init()),
        })
    }

    pub fn base(&self) -> &AggregateRoot<SubscriptionId> {
        &self.base
    }

    pub fn plan(&self) -> &SubscriptionPlan {
        &self.plan
    }

    pub fn payments(&self) -> &[Payment] {
        &self.payments
    }

    pub fn status_history(&self) -> &StatusHistory<Status> {
        &self.status_history
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status_history.current(), Status::Active)
            && if let Some(payment) = self.payments.last() {
                payment.is_current()
            } else {
                false
            }
    }

    pub fn require_payment(&mut self) -> Result<()> {
        if self.is_active() {
            return Err(Error::new("subscription", "already_paid"));
        }

        let status = self.status_history.current().wait_for_payment()?;
        self.status_history.add_status(status);

        Ok(())
    }

    pub fn add_payment(&mut self, payment: Payment) -> Result<()> {
        let status = self.status_history.current().pay()?;
        self.status_history.add_status(status);

        self.payments.push(payment);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    

    

    use chrono::Utc;

    use crate::mocks;

    #[test]
    fn create() {
        let subscription = mocks::subscription("sub-1", "user-1", "plan-1", 45.0);
        assert_eq!(subscription.plan().price(), 45.0);
        assert_eq!(subscription.payments().len(), 0);
        assert_eq!(
            subscription.status_history().current().to_string(),
            "waiting-payment"
        );
        assert_eq!(subscription.is_active(), false);
    }

    #[test]
    fn free_plan() {
        let subscription = mocks::subscription("sub-1", "user-1", "plan-1", 0.0);
        assert_eq!(subscription.plan().price(), 0.0);
        assert_eq!(subscription.payments().len(), 0);
        assert_eq!(
            subscription.status_history().current().to_string(),
            "active"
        );
        assert_eq!(subscription.is_active(), true);
    }

    #[test]
    fn add_payment() {
        let mut subscription = mocks::subscription("sub-1", "user-1", "plan-1", 75.0);
        assert_eq!(subscription.payments().len(), 0);

        let payment = mocks::payment("income", 85.0, Utc::now());
        assert!(subscription.add_payment(payment).is_err());

        let payment = mocks::payment("income", 75.0, Utc::now());
        assert!(subscription.add_payment(payment).is_ok());

        assert_eq!(
            subscription.status_history().current().to_string(),
            "active"
        );
    }
}
