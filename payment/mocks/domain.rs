use std::str::FromStr;

use chrono::{DateTime, Utc};

use identity::domain::user::UserId;

use crate::domain::payment::{Amount, Kind, Payment};
use crate::domain::plan::{Plan, PlanId, Price};
use crate::domain::subscription::{Subscription, SubscriptionId};

pub fn subscription(sub_id: &str, user_id: &str, plan_id: &str, plan_price: f64) -> Subscription {
    Subscription::new(
        SubscriptionId::new(sub_id).unwrap(),
        UserId::new(user_id).unwrap(),
        Plan::new(
            PlanId::new(plan_id).unwrap(),
            Price::new(plan_price).unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}

pub fn payment(kind: &str, amount: f64, _date: DateTime<Utc>) -> Payment {
    Payment::new(Kind::from_str(kind).unwrap(), Amount::new(amount).unwrap()).unwrap()
}
