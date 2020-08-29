mod status;
pub use status::*;

use chrono::{DateTime, Utc};

use common::event::Event;
use common::model::{AggregateRoot, StatusHistory, StringId};
use common::result::Result;

use crate::domain::payment::Payment;
use crate::domain::plan::Plan;

pub type SubscriptionId = StringId;

#[derive(Debug, Clone)]
pub struct Subscription {
    base: AggregateRoot<SubscriptionId, Event>,
    plan: Plan,
    date: DateTime<Utc>,
    payments: Vec<Payment>,
    status_history: StatusHistory<Status>,
}

impl Subscription {
    pub fn new(id: SubscriptionId, plan: Plan) -> Result<Self> {
        Ok(Subscription {
            base: AggregateRoot::new(id),
            plan,
            date: Utc::now(),
            payments: Vec::new(),
            status_history: StatusHistory::new(Status::Active),
        })
    }

    pub fn base(&self) -> &AggregateRoot<SubscriptionId, Event> {
        &self.base
    }

    pub fn plan(&self) -> &Plan {
        &self.plan
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn payments(&self) -> &[Payment] {
        &self.payments
    }

    pub fn status_history(&self) -> &StatusHistory<Status> {
        &self.status_history
    }

    pub fn set_plan(&mut self, plan: Plan) {
        self.plan = plan;
    }
}
