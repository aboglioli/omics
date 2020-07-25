use common::error::Error;
use common::model::AggregateRoot;

pub type PlanID = String;

pub struct Plan {
    base: AggregateRoot<PlanID>,
}
