mod price;
mod repository;
pub use price::*;
pub use repository::*;

use common::model::{AggregateRoot, Events, StringId};
use common::result::Result;
use shared::event::PlanEvent;

pub type PlanId = StringId;

#[derive(Debug, Clone)]
pub struct Plan {
    base: AggregateRoot<PlanId>,
    events: Events<PlanEvent>,
    price: Price,
}

impl Plan {
    pub fn new(id: PlanId, price: Price) -> Result<Self> {
        let mut plan = Plan {
            base: AggregateRoot::new(id),
            events: Events::new(),
            price,
        };

        plan.events.record_event(PlanEvent::Created {
            id: plan.base().id().to_string(),
            price: plan.price().value(),
        });

        Ok(plan)
    }

    pub fn build(base: AggregateRoot<PlanId>, price: Price) -> Self {
        Plan {
            base,
            events: Events::new(),
            price,
        }
    }

    pub fn base(&self) -> &AggregateRoot<PlanId> {
        &self.base
    }

    pub fn events(&self) -> &Events<PlanEvent> {
        &self.events
    }

    pub fn price(&self) -> &Price {
        &self.price
    }

    pub fn change_price(&mut self, price: Price) -> Result<()> {
        self.price = price;

        self.events.record_event(PlanEvent::PriceChanged {
            id: self.base().id().to_string(),
            price: self.price().value(),
        });

        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.base.delete();

        self.events.record_event(PlanEvent::Deleted {
            id: self.base().id().to_string(),
        });

        Ok(())
    }
}
