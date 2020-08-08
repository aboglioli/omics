use common::event::Event;
use common::model::AggregateRoot;

pub type PlanId = String;

pub struct Plan {
    base: AggregateRoot<PlanId, Event>,
}

impl Plan {
    pub fn base(&self) -> &AggregateRoot<PlanId, Event> {
        &self.base
    }
}
