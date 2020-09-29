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
    pub fn new(id: SubscriptionId, user_id: UserId, plan: SubscriptionPlan) -> Result<Self> {
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

    pub fn pay(&mut self, payment: Payment) -> Result<()> {
        let status = self.status_history.current().pay()?;
        self.status_history.add_status(status);

        self.payments.push(payment);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{Duration, TimeZone, Utc};

    use crate::domain::plan::{Plan, PlanId, Price};
    use crate::domain::payment::{Payment, PaymentId, Amount};
    use crate::mocks;

    fn subscription(sub_id: &str, user_id: &str, plan_id: &str, plan_price: f64) -> Subscription {
        Subscription::new(
            SubscriptionId::new(sub_id).unwrap(),
            UserId::new(user_id).unwrap(),
            SubscriptionPlan::new(
                Plan::new(
                    PlanId::new(plan_id).unwrap(),
                    Price::new(plan_price).unwrap(),
                )
                .unwrap(),
            )
            .unwrap(),
        )
        .unwrap()
    }

    fn payment(payment_id: &str, amount: f64) -> Payment {
        Payment::new(
            PaymentId::new(payment_id).unwrap(),
            Amount::new(amount).unwrap(),
        ).unwrap()
    }

    #[test]
    fn create() {
        let subscription = subscription("sub-1", "user-1", "plan-1", 45.0);
        assert_eq!(subscription.plan().price(), 45.0);
        assert_eq!(subscription.payments().len(), 0);
        assert_eq!(subscription.status_history().current().to_string(), "waiting-payment");
        assert_eq!(subscription.is_active(), false);
    }

    #[test]
    fn free_plan() {
        let subscription = subscription("sub-1", "user-1", "plan-1", 0.0);
        assert_eq!(subscription.plan().price(), 0.0);
        assert_eq!(subscription.payments().len(), 0);
        assert_eq!(subscription.status_history().current().to_string(), "active");
        assert_eq!(subscription.is_active(), true);
    }

    #[test]
    fn add_payment() {
        let mut subscription = subscription("sub-1", "user-1", "plan-1", 75.0);
        assert_eq!(subscription.payments().len(), 0);

        let mut payment = payment("payment-1", 85.0);
        assert!(subscription.add_payment(payment).is_err());

        let mut payment = payment("payment-1", 75.0);
        assert!(subscription.add_payment(payment).is_ok());

        assert_eq!(subscription.status_history().current().to_string(), "active");
    }
}
