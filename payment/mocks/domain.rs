use std::str::FromStr;

use chrono::{DateTime, Utc};

use publishing::domain::reader::{Reader, ReaderId};

use crate::domain::payment::{Amount, Kind, Payment};
use crate::domain::plan::{Plan, PlanId, Price};
use crate::domain::subscription::{Subscription, SubscriptionId};

#[allow(dead_code)]
pub fn subscription(sub_id: &str, user_id: &str, plan_id: &str, plan_price: f64) -> Subscription {
    Subscription::new(
        SubscriptionId::new(sub_id).unwrap(),
        &Reader::new(ReaderId::new(user_id).unwrap(), "username").unwrap(),
        plan(plan_id, plan_price),
    )
    .unwrap()
}

#[allow(dead_code)]
pub fn payment(kind: &str, amount: f64, _date: DateTime<Utc>) -> Payment {
    Payment::new(Kind::from_str(kind).unwrap(), Amount::new(amount).unwrap()).unwrap()
}

#[allow(dead_code)]
pub fn plan(id: &str, price: f64) -> Plan {
    Plan::new(
        PlanId::new(id).unwrap(),
        "basic".to_owned(),
        "Basic".to_owned(),
        Price::new(price).unwrap(),
    )
    .unwrap()
}
