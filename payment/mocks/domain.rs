use chrono::{DateTime, Duration, Utc};

use common::model::{AggregateRoot, StatusHistory, StatusItem};

use crate::domain::payment::{Amount, Payment, PaymentId, Status as PaymentStatus};
use crate::domain::plan::{Plan, PlanId, Price};
use crate::domain::subscription::{Subscription, SubscriptionId, SubscriptionPlan};
use crate::domain::user::{User, UserId};

pub fn subscription1() -> Subscription {
    Subscription::new(
        SubscriptionId::new("subscription-1").unwrap(),
        user1().base().id().clone(),
        SubscriptionPlan::new(plan1()).unwrap(),
    )
    .unwrap()
}

pub fn subscription2() -> Subscription {
    Subscription::new(
        SubscriptionId::new("subscription-2").unwrap(),
        user2().base().id().clone(),
        SubscriptionPlan::new(plan2()).unwrap(),
    )
    .unwrap()
}

pub fn subscription3() -> Subscription {
    Subscription::new(
        SubscriptionId::new("subscription-1").unwrap(),
        user1().base().id().clone(),
        SubscriptionPlan::new(plan3()).unwrap(),
    )
    .unwrap()
}

pub fn user1() -> User {
    User::new(UserId::new("user-1").unwrap()).unwrap()
}

pub fn user2() -> User {
    User::new(UserId::new("user-2").unwrap()).unwrap()
}

pub fn plan1() -> Plan {
    Plan::new(PlanId::new("plan-1").unwrap(), Price::new(120.0).unwrap()).unwrap()
}

pub fn plan2() -> Plan {
    Plan::new(PlanId::new("plan-2").unwrap(), Price::new(150.0).unwrap()).unwrap()
}

pub fn plan3() -> Plan {
    Plan::new(PlanId::new("plan-3").unwrap(), Price::new(0.0).unwrap()).unwrap()
}

pub fn payment1(amount: f64, paid_at: DateTime<Utc>) -> Payment {
    let created_at = paid_at - Duration::minutes(30);
    Payment::build(
        AggregateRoot::build(PaymentId::new("payment-1").unwrap(), created_at, None, None),
        Amount::new(amount).unwrap(),
        StatusHistory::build(vec![
            StatusItem::build(PaymentStatus::WaitingPayment, created_at),
            StatusItem::build(PaymentStatus::Paid, paid_at),
        ]),
    )
}
