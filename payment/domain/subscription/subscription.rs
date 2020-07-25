use chrono::{DateTime, Utc};

use common::error::Error;
use common::model::{AggregateRoot, StatusHistory};

use crate::domain::plan::PlanId;
use crate::domain::subscription::{SubscriptionPayment, SubscriptionPlan, SubscriptionStatus};
use crate::domain::user::UserId;

pub type SubscriptionId = String;

pub struct Subscription {
    base: AggregateRoot<SubscriptionId>,
    subscribed_at: DateTime<Utc>,
    user_id: UserId,
    plan: SubscriptionPlan,
    status: StatusHistory<SubscriptionStatus, String>,
    payments: Vec<SubscriptionPayment>,
}

impl Subscription {
    pub fn new(
        id: SubscriptionId,
        user_id: UserId,
        plan_id: PlanId,
    ) -> Result<Subscription, Error> {
        Ok(Subscription {
            base: AggregateRoot::new(id),
            subscribed_at: Utc::now(),
            user_id,
            plan: SubscriptionPlan::new(plan_id)?,
            status: StatusHistory::init(SubscriptionStatus::Active),
            payments: Vec::new(),
        })
    }

    pub fn base(&self) -> &AggregateRoot<SubscriptionId> {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut AggregateRoot<SubscriptionId> {
        &mut self.base
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

    pub fn confirm_plan(&mut self) -> Result<(), Error> {
        self.plan = self.plan.confirm()?;
        Ok(())
    }

    pub fn change_plan(&mut self, plan_id: PlanId) -> Result<(), Error> {
        self.plan = SubscriptionPlan::new(plan_id)?;
        Ok(())
    }
}
