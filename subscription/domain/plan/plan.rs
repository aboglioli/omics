use common::model::{AggregateRoot, DefaultEvent};

pub type PlanId = String;

pub struct Plan {
    base: AggregateRoot<PlanId, DefaultEvent>,
}
