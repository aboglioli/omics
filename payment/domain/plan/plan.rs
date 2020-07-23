use common::error::Error;
use common::model::{Entity, ID};

pub type PlanID = String;

pub struct Plan {
    id: ID<PlanID>,
}

impl Entity<PlanID> for Plan {
    fn id(&self) -> &ID<PlanID> {
        &self.id
    }
}
