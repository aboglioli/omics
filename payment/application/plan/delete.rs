use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::plan::{PlanId, PlanRepository};
use crate::domain::subscription::SubscriptionRepository;

pub struct Delete<'a> {
    event_pub: &'a dyn EventPublisher,

    plan_repo: &'a dyn PlanRepository,
    subscription_repo: &'a dyn SubscriptionRepository,
}

impl<'a> Delete<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        plan_repo: &'a dyn PlanRepository,
        subscription_repo: &'a dyn SubscriptionRepository,
    ) -> Self {
        Delete {
            event_pub,
            plan_repo,
            subscription_repo,
        }
    }

    pub async fn exec(
        &self,
        (_auth_id, auth_role): UserIdAndRole,
        plan_id: String,
    ) -> Result<CommandResponse> {
        if !auth_role.can("delete_plan") {
            return Err(Error::unauthorized());
        }

        let plan_id = PlanId::new(plan_id)?;

        let p_subscriptions = self
            .subscription_repo
            .search(None, Some(&plan_id), None, None, None, None, None, None)
            .await?;
        if p_subscriptions.matching_criteria() > 0 {
            return Err(Error::new("plan", "existing_subscriptions"));
        }

        let mut plan = self.plan_repo.find_by_id(&plan_id).await?;

        plan.delete()?;

        self.plan_repo.delete(plan.base().id()).await?;

        self.event_pub.publish_all(plan.events().to_vec()?).await?;

        Ok(CommandResponse::default())
    }
}
