use chrono::{DateTime, Utc};

use common::event::Event;
use common::model::{AggregateRoot, StatusHistory};
use common::result::Result;

use crate::domain::plan::PlanId;
use crate::domain::subscription::{SubscriptionPlan, SubscriptionStatus};
use crate::domain::user::UserId;

pub type SubscriptionId = String;

pub struct Subscription {
    base: AggregateRoot<SubscriptionId, Event>,
    subscribed_at: DateTime<Utc>,
    user_id: UserId,
    plan: SubscriptionPlan,
    status: StatusHistory<SubscriptionStatus, String>,
    payments: Vec<String>,
}

impl Subscription {
    pub fn new(id: SubscriptionId, user_id: UserId, plan_id: PlanId) -> Result<Subscription> {
        Ok(Subscription {
            base: AggregateRoot::new(id),
            subscribed_at: Utc::now(),
            user_id,
            plan: SubscriptionPlan::new(plan_id)?,
            status: StatusHistory::new(SubscriptionStatus::Active),
            payments: Vec::new(),
        })
    }

    pub fn base(&self) -> &AggregateRoot<SubscriptionId, Event> {
        &self.base
    }

    pub fn subscribed_at(&self) -> &DateTime<Utc> {
        &self.subscribed_at
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn plan(&self) -> &SubscriptionPlan {
        &self.plan
    }

    pub fn status(&self) -> &StatusHistory<SubscriptionStatus, String> {
        &self.status
    }

    pub fn payments(&self) -> &[String] {
        &self.payments
    }

    pub fn confirm_plan(&mut self) -> Result<()> {
        self.plan = self.plan.confirm()?;
        Ok(())
    }

    pub fn change_plan(&mut self, plan_id: PlanId) -> Result<()> {
        self.plan = SubscriptionPlan::new(plan_id)?;
        Ok(())
    }
}
