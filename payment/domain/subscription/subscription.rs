use common::error::Error;
use common::model::AggregateRoot;

use crate::domain::plan::PlanID;
use crate::domain::user::UserID;

pub type SubscriptionID = String;

pub struct Subscription {
    base: AggregateRoot<SubscriptionID>,
    user_id: UserID,
    plan_id: PlanID,
}

impl Subscription {
    pub fn new(
        id: SubscriptionID,
        user_id: UserID,
        plan_id: PlanID,
    ) -> Result<Subscription, Error> {
        Ok(Subscription {
            base: AggregateRoot::new(id),
            user_id,
            plan_id,
        })
    }
}
