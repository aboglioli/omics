use common::error::Error;
use common::model::AggregateRoot;

pub type PlanId = String;

pub struct Plan {
    base: AggregateRoot<PlanId>,
}
