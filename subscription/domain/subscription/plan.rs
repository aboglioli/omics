use chrono::{DateTime, Utc};

use common::result::Result;

use crate::domain::plan::PlanId;

pub enum SubscriptionPlanStatus {
    AssignedAt(DateTime<Utc>),
    ConfirmedAt(DateTime<Utc>),
}

pub struct SubscriptionPlan {
    id: PlanId,
    status: SubscriptionPlanStatus,
}

impl SubscriptionPlan {
    pub fn new(id: PlanId) -> Result<SubscriptionPlan> {
        Ok(SubscriptionPlan {
            id,
            status: SubscriptionPlanStatus::AssignedAt(Utc::now()),
        })
    }

    pub fn id(&self) -> &PlanId {
        &self.id
    }

    pub fn status_history(&self) -> &SubscriptionPlanStatus {
        &self.status
    }

    pub fn is_confirmed(&self) -> bool {
        match self.status {
            SubscriptionPlanStatus::ConfirmedAt(_) => true,
            _ => false,
        }
    }

    pub fn confirm(&self) -> Result<SubscriptionPlan> {
        Ok(SubscriptionPlan {
            id: self.id.clone(),
            status: SubscriptionPlanStatus::ConfirmedAt(Utc::now()),
        })
    }
}
