use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use common::result::Result;

use crate::domain::plan::{Plan, PlanId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    plan_id: PlanId,
    price: f64,
    assigned_at: DateTime<Utc>,
}

impl SubscriptionPlan {
    pub fn new(plan: Plan) -> Result<Self> {
        Ok(SubscriptionPlan {
            plan_id: plan.base().id().clone(),
            price: plan.price().value(),
            assigned_at: Utc::now(),
        })
    }

    pub fn build(plan_id: PlanId, price: f64, assigned_at: DateTime<Utc>) -> Self {
        SubscriptionPlan {
            plan_id,
            price,
            assigned_at,
        }
    }

    pub fn plan_id(&self) -> &PlanId {
        &self.plan_id
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn assigned_at(&self) -> &DateTime<Utc> {
        &self.assigned_at
    }
}
