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
use publishing::domain::reader::Reader;
use shared::event::SubscriptionEvent;

use crate::domain::payment::{Amount, Kind, Payment};
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
    pub fn new(id: SubscriptionId, reader: &Reader, plan: Plan) -> Result<Self> {
        if reader.is_subscribed() {
            return Err(Error::new("user", "already_subscribed"));
        }

        let plan = SubscriptionPlan::new(plan)?;
        let mut status_history = StatusHistory::new(Status::init());

        if plan.price() == 0.0 {
            let status = status_history.current().pay()?;
            status_history.add_status(status);
        }

        let mut subscription = Subscription {
            base: AggregateRoot::new(id),
            events: Events::new(),
            user_id: reader.base().id().clone(),
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

        if self.plan.price() == 0.0 {
            let status = self.status_history.current().pay()?;
            self.status_history.add_status(status);
        }

        self.events.record_event(SubscriptionEvent::PlanChanged {
            id: self.base().id().to_string(),
            user_id: self.user_id().to_string(),
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
                user_id: self.user_id().to_string(),
            });

        Ok(())
    }

    pub fn pay(&mut self) -> Result<Payment> {
        if self.is_active() {
            return Err(Error::new("subscription", "already_paid"));
        }

        let payment = Payment::new(Kind::Income, Amount::new(self.plan.price())?)?;

        let status = self.status_history.current().pay()?;
        self.status_history.add_status(status);

        self.payments.push(payment.clone());

        self.events.record_event(SubscriptionEvent::PaymentAdded {
            id: self.base().id().to_string(),
            user_id: self.user_id().to_string(),
            amount: payment.amount().value(),
        });

        Ok(payment)
    }

    pub fn disable(&mut self) -> Result<()> {
        let status = self.status_history.current().close()?;
        self.status_history.add_status(status);

        self.base.delete();

        self.events.record_event(SubscriptionEvent::Disabled {
            id: self.base().id().to_string(),
            user_id: self.user_id().to_string(),
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    use chrono::DateTime;

    use common::model::StatusItem;

    use crate::domain::payment::{Amount, Kind};
    use crate::domain::plan::{PlanId, Price};
    use crate::mocks;

    #[test]
    fn create() {
        let subscription = mocks::subscription("sub-1", "user-1", "plan-1", 45.0);
        assert_eq!(subscription.plan().price(), 45.0);
        assert_eq!(subscription.payments().len(), 0);
        assert_eq!(
            subscription.status_history().current().to_string(),
            "waiting-for-payment"
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
    fn pay() {
        let mut subscription = mocks::subscription("sub-1", "user-1", "plan-1", 75.0);
        assert!(!subscription.is_active());
        assert_eq!(subscription.payments().len(), 0);
        assert!(subscription.pay().is_ok());
        assert!(subscription.pay().is_err());
        assert_eq!(subscription.payments().len(), 1);
        assert!(subscription.is_active());
        assert_eq!(
            subscription.status_history().current().to_string(),
            "active"
        );

        assert!(!subscription.events().to_vec().unwrap().is_empty());
    }

    #[test]
    fn subscription_status_from_payments_and_change_plan() {
        let mut subscription = Subscription::build(
            AggregateRoot::new(SubscriptionId::new("#sub01").unwrap()),
            UserId::new("#user01").unwrap(),
            SubscriptionPlan::build(
                PlanId::new("#plan01").unwrap(),
                140.0,
                DateTime::from_str("2020-05-01T14:30:00Z").unwrap(),
            ),
            vec![
                Payment::build(
                    Kind::Income,
                    Amount::new(140.0).unwrap(),
                    DateTime::from_str("2020-05-01T14:30:00Z").unwrap(),
                ),
                Payment::build(
                    Kind::Income,
                    Amount::new(140.0).unwrap(),
                    DateTime::from_str("2020-06-01T14:30:00Z").unwrap(),
                ),
                Payment::build(
                    Kind::Income,
                    Amount::new(140.0).unwrap(),
                    DateTime::from_str("2020-07-01T14:30:00Z").unwrap(),
                ),
                Payment::build(
                    Kind::Income,
                    Amount::new(140.0).unwrap(),
                    DateTime::from_str("2020-08-01T14:30:00Z").unwrap(),
                ),
            ],
            StatusHistory::build(vec![
                StatusItem::new(Status::init()),
                StatusItem::new(Status::Active),
            ]),
        );

        assert!(!subscription.is_active());
        assert!(subscription.require_payment().is_ok());
        assert!(subscription.require_payment().is_err());

        let payment = subscription.pay().unwrap();
        assert_eq!(payment.amount().value(), 140.0);
        assert!(subscription.is_active());
        assert!(subscription.pay().is_err());
        assert!(subscription.require_payment().is_err());
        assert_eq!(subscription.payments().len(), 5);

        assert!(!subscription.events().to_vec().unwrap().is_empty());

        assert!(subscription
            .change_plan(
                Plan::new(PlanId::new("#plan02").unwrap(), Price::new(175.75).unwrap(),).unwrap(),
            )
            .is_ok());
    }
}
