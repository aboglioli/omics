mod payment;
mod plan;
mod repository;
mod status;
pub use self::payment::*;
pub use plan::*;
pub use repository::*;
pub use status::*;

use common::error::Error;

use common::model::{AggregateRoot, StatusHistory, StringId};
use common::result::Result;

use crate::domain::user::UserId;

pub type SubscriptionId = StringId;

#[derive(Debug, Clone)]
pub struct Subscription {
    base: AggregateRoot<SubscriptionId>,
    user_id: UserId,
    plan: SubscriptionPlan,
    payments: Vec<SubscriptionPayment>,
    status_history: StatusHistory<Status>,
}

impl Subscription {
    pub fn new(id: SubscriptionId, user_id: UserId, plan: SubscriptionPlan) -> Result<Self> {
        Ok(Subscription {
            base: AggregateRoot::new(id),
            user_id,
            plan,
            payments: Vec::new(),
            status_history: StatusHistory::new(Status::WaitingPayment),
        })
    }

    pub fn base(&self) -> &AggregateRoot<SubscriptionId> {
        &self.base
    }

    pub fn plan(&self) -> &SubscriptionPlan {
        &self.plan
    }

    pub fn payments(&self) -> &[SubscriptionPayment] {
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

    pub fn set_plan(&mut self, plan: SubscriptionPlan) {
        self.plan = plan;
    }

    pub fn require_payment(&mut self) -> Result<()> {
        if self.is_active() {
            return Err(Error::new("subscription", "already_paid"));
        }

        let status = self.status_history.current().prepare_for_payment()?;
        self.status_history.add_status(status);

        Ok(())
    }

    pub fn pay(&mut self, payment: SubscriptionPayment) -> Result<()> {
        let status = self.status_history.current().pay()?;
        self.status_history.add_status(status);

        self.payments.push(payment);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;
    use chrono::{Duration, TimeZone, Utc};

    #[test]
    fn create() {
        let subscription = mocks::subscription1();
        assert_eq!(subscription.plan().price(), 120.0);
        assert_eq!(subscription.payments().len(), 0);
        assert_eq!(
            subscription.status_history().current().to_string(),
            "waiting-payment"
        );
        assert_eq!(subscription.is_active(), false);
    }

    #[test]
    fn free_plan() {
        let mut subscription = mocks::subscription3();

        let payment1 = mocks::payment1(0.0, Utc::now());
        let payment1 = SubscriptionPayment::new(payment1).unwrap();
        assert!(subscription.pay(payment1).is_ok());

        assert_eq!(subscription.is_active(), true);
    }

    #[test]
    fn unnecessary_payment() {
        let mut subscription = mocks::subscription1();

        let now = Utc::now() - Duration::days(15);
        let payment1 = mocks::payment1(150.0, now);
        let payment1 = SubscriptionPayment::new(payment1).unwrap();
        assert!(subscription.pay(payment1).is_ok());

        let payment2 = mocks::payment1(150.0, Utc::now());
        let payment2 = SubscriptionPayment::new(payment2).unwrap();
        assert!(subscription.require_payment().is_err());
        assert!(subscription.pay(payment2).is_err());
    }

    #[test]
    fn payment() {
        let mut subscription = mocks::subscription1();

        let payment1 = mocks::payment1(150.0, Utc.ymd(2020, 7, 2).and_hms(16, 0, 0));
        let payment1 = SubscriptionPayment::new(payment1).unwrap();
        assert!(subscription.pay(payment1).is_ok());

        let payment2 = mocks::payment1(150.0, Utc.ymd(2020, 8, 2).and_hms(16, 0, 0));
        let payment2 = SubscriptionPayment::new(payment2).unwrap();
        assert!(subscription.require_payment().is_ok());
        assert!(subscription.pay(payment2.clone()).is_ok());
        assert!(subscription.pay(payment2).is_err());
        assert_eq!(subscription.is_active(), false);

        let payment3 = mocks::payment1(150.0, Utc.ymd(2020, 9, 2).and_hms(16, 0, 0));
        let payment3 = SubscriptionPayment::new(payment3).unwrap();
        assert!(subscription.require_payment().is_ok());
        assert!(subscription.pay(payment3).is_ok());
        assert_eq!(subscription.is_active(), true);
    }
}
