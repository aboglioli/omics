use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::domain::user::UserRepository;
use identity::UserIdAndRole;

use crate::domain::plan::{PlanId, PlanRepository, Price};

#[derive(Deserialize)]
pub struct UpdateCommand {
    name: String,
    description: String,
    price: f64,
}

pub struct Update<'a> {
    event_pub: &'a dyn EventPublisher,

    plan_repo: &'a dyn PlanRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Update<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        plan_repo: &'a dyn PlanRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Update {
            event_pub,
            plan_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        (_auth_id, auth_role): UserIdAndRole,
        plan_id: String,
        cmd: UpdateCommand,
    ) -> Result<CommandResponse> {
        if !auth_role.can("update_plan") {
            return Err(Error::unauthorized());
        }

        let mut plan = self.plan_repo.find_by_id(&PlanId::new(plan_id)?).await?;

        plan.set_name(cmd.name)?;
        plan.set_description(cmd.description)?;
        plan.change_price(Price::new(cmd.price)?)?;

        self.plan_repo.save(&mut plan).await?;

        self.event_pub.publish_all(plan.events().to_vec()?).await?;

        Ok(CommandResponse::default())
    }
}
