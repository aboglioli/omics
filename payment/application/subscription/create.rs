use common::request::CommandResponse;
use common::result::Result;

use crate::domain::plan::{PlanId, PlanRepository};
use crate::domain::subscription::{Subscription, SubscriptionPlan, SubscriptionRepository};
use crate::domain::user::{UserId, UserRepository};

pub struct Create<'a> {
    plan_repo: &'a dyn PlanRepository,
    subscription_repo: &'a dyn SubscriptionRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Create<'a> {
    pub fn new(
        plan_repo: &'a dyn PlanRepository,
        subscription_repo: &'a dyn SubscriptionRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Create {
            plan_repo,
            subscription_repo,
            user_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, plan_id: String) -> Result<CommandResponse> {
        let user_id = UserId::new(auth_id)?;
        self.user_repo.find_by_id(&user_id).await?;

        let plan_id = PlanId::new(plan_id)?;
        let plan = self.plan_repo.find_by_id(&plan_id).await?;
        let plan = SubscriptionPlan::new(plan)?;

        let _subscription =
            Subscription::new(self.subscription_repo.next_id().await?, user_id, plan)?;

        Ok(CommandResponse::default())
    }
}
