use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};

use crate::domain::plan::{Plan, PlanId, PlanRepository, Price};

pub struct Delete<'a> {
    event_pub: &'a dyn EventPublisher,

    plan_repo: &'a dyn PlanRepository,
    user_repo: &'a dyn UserRepository,
}

// TODO: update subscriptions before deleting
impl<'a> Delete<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        plan_repo: &'a dyn PlanRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Delete {
            event_pub,
            plan_repo,
            user_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, plan_id: String) -> Result<CommandResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

        let mut plan = self.plan_repo.find_by_id(&PlanId::new(plan_id)?).await?;

        plan.delete()?;

        self.plan_repo.delete(plan.base().id()).await?;

        self.event_pub.publish_all(plan.events().to_vec()?).await?;

        Ok(CommandResponse::default())
    }
}
