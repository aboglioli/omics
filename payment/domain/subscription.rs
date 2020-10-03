mod plan;
mod repository;
mod status;
pub use plan::*;
pub use repository::*;
pub use status::*;

use common::error::Error;
use common::model::{AggregateRoot, Events, StatusHistory, StringId};
use common::result::Result;
use identity::domain::user::UserId;
use shared::event::SubscriptionEvent;

use crate::domain::payment::Payment;
use crate::domain::plan::Plan;

pub type SubscriptionId = StringId;

#[derive(Debug, Clone)]
pub struct Subscription {
    base: AggregateRoot<SubscriptionId>,
    events: Events<SubscriptionEvent>,
    user_id: UserId,
    plan: SubscriptionPlan,
    payments: Vec<Payment>,
    status_history: StatusHistory<Status>,
}

impl Subscription {
    pub fn new(id: SubscriptionId, user_id: UserId, plan: Plan) -> Result<Self> {
        let plan = SubscriptionPlan::new(plan)?;
        let mut status_history = StatusHistory::new(Status::init());

        if plan.price() == 0.0 {
            let status = status_history.current().pay()?;
            status_history.add_status(status);
        }

        let mut subscription = Subscription {
            base: AggregateRoot::new(id),
            events: Events::new(),
            user_id,
            plan,
            payments: Vec::new(),
            status_history,
        };

        subscription
            .events
            .record_event(SubscriptionEvent::Created {
                id: subscription.base().id().to_string(),
                user_id: subscription.user_id().to_string(),
                plan_id: subscription.plan().plan_id().to_string(),
            });

        Ok(subscription)
    }

    pub fn build(
        base: AggregateRoot<SubscriptionId>,
        user_id: UserId,
        plan: SubscriptionPlan,
        payments: Vec<Payment>,
        status_history: StatusHistory<Status>,
    ) -> Self {
        Subscription {
            base,
            events: Events::new(),
            user_id,
            plan,
            payments,
            status_history,
        }
    }

    pub fn base(&self) -> &AggregateRoot<SubscriptionId> {
        &self.base
    }

    pub fn events(&self) -> &Events<SubscriptionEvent> {
        &self.events
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
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
        if self.plan().price() == 0.0 {
            return true;
        }

        matches!(self.status_history.current(), Status::Active)
            && if let Some(payment) = self.payments.last() {
                payment.is_current(30)
            } else {
                false
            }
    }

    pub fn change_plan(&mut self, plan: Plan) -> Result<()> {
        let plan = SubscriptionPlan::new(plan)?;
        self.plan = plan;

        self.events.record_event(SubscriptionEvent::PlanChanged {
            id: self.base().id().to_string(),
            plan_id: self.plan().plan_id().to_string(),
        });

        Ok(())
    }

    pub fn require_payment(&mut self) -> Result<()> {
        if self.is_active() {
            return Err(Error::new("subscription", "already_paid"));
        }

        let status = self.status_history.current().wait_for_payment()?;
        self.status_history.add_status(status);

        self.events
            .record_event(SubscriptionEvent::PaymentRequired {
                id: self.base().id().to_string(),
            });

        Ok(())
    }

    pub fn add_payment(&mut self, payment: Payment) -> Result<()> {
        if self.is_active() {
            return Err(Error::new("subscription", "already_paid"));
        }

        if self.plan.price() != payment.amount().value() {
            return Err(Error::new("subscription", "payment_differente_to_plan"));
        }

        let status = self.status_history.current().pay()?;
        self.status_history.add_status(status);

        let amount = payment.amount().value();

        self.payments.push(payment);

        self.events.record_event(SubscriptionEvent::PaymentAdded {
            id: self.base().id().to_string(),
            amount,
        });

        Ok(())
    }

    pub fn disable(&mut self) -> Result<()> {
        let status = self.status_history.current().close()?;
        self.status_history.add_status(status);

        self.base.delete();

        self.events.record_event(SubscriptionEvent::Disabled {
            id: self.base().id().to_string(),
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{Duration, Utc};

    use crate::domain::payment::{Amount, Kind};
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

        assert!(!subscription.events().to_vec().unwrap().is_empty());
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

        assert!(!subscription.events().to_vec().unwrap().is_empty());
    }

    #[test]
    fn subscription_status_from_payments() {
        let mut subscription = mocks::subscription("sub-1", "user-1", "plan-1", 75.0);
        assert!(subscription
            .add_payment(Payment::build(
                Kind::Income,
                Amount::new(75.0).unwrap(),
                Utc::now() - Duration::days(70),
            ))
            .is_ok());
        assert!(!subscription.is_active());

        assert!(subscription.require_payment().is_ok());
        assert!(subscription.require_payment().is_err());

        assert!(subscription
            .add_payment(Payment::build(
                Kind::Income,
                Amount::new(50.0).unwrap(),
                Utc::now() - Duration::days(70),
            ))
            .is_err());

        assert!(subscription
            .add_payment(Payment::build(
                Kind::Income,
                Amount::new(75.0).unwrap(),
                Utc::now() - Duration::days(40),
            ))
            .is_ok());
        assert!(!subscription.is_active());

        assert!(subscription
            .add_payment(Payment::build(
                Kind::Income,
                Amount::new(75.0).unwrap(),
                Utc::now() - Duration::days(10),
            ))
            .is_ok());
        assert!(subscription.is_active());

        assert!(subscription
            .add_payment(Payment::build(
                Kind::Income,
                Amount::new(50.0).unwrap(),
                Utc::now(),
            ))
            .is_err());

        assert!(subscription.require_payment().is_err());

        assert_eq!(subscription.payments().len(), 3);

        assert!(!subscription.events().to_vec().unwrap().is_empty());
    }

    #[test]
    fn change_plan() {
        let mut subscription = mocks::subscription("sub-1", "user-1", "plan-1", 0.0);
        assert!(subscription
            .add_payment(Payment::new(Kind::Income, Amount::new(50.0).unwrap()).unwrap())
            .is_err());

        assert!(subscription
            .change_plan(mocks::plan("plan-2", 45.0))
            .is_ok());

        assert!(subscription
            .add_payment(Payment::new(Kind::Income, Amount::new(50.0).unwrap()).unwrap())
            .is_err());
        assert!(subscription
            .add_payment(Payment::new(Kind::Income, Amount::new(45.0).unwrap()).unwrap())
            .is_ok());
        assert!(subscription
            .add_payment(Payment::new(Kind::Income, Amount::new(45.0).unwrap()).unwrap())
            .is_err());
    }
}
