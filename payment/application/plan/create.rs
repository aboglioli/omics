use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};

use crate::domain::plan::{Plan, PlanId, PlanRepository, Price};

#[derive(Deserialize)]
pub struct CreateCommand {
    id: String,
    name: String,
    description: String,
    price: f64,
}

pub struct Create<'a> {
    event_pub: &'a dyn EventPublisher,

    plan_repo: &'a dyn PlanRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Create<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        plan_repo: &'a dyn PlanRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Create {
            event_pub,
            plan_repo,
            user_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, cmd: CreateCommand) -> Result<CommandResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

        let plan_id = PlanId::new(cmd.id)?;

        if self.plan_repo.find_by_id(&plan_id).await.is_ok() {
            return Err(Error::new("plan", "already_exists"));
        }

        let mut plan = Plan::new(plan_id, cmd.name, cmd.description, Price::new(cmd.price)?)?;

        self.plan_repo.save(&mut plan).await?;

        self.event_pub.publish_all(plan.events().to_vec()?).await?;

        Ok(CommandResponse::default())
    }
}
