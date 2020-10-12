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
    name: String,
    description: String,
    price: Price,
}

impl Plan {
    pub fn new<S: Into<String>>(id: PlanId, name: S, description: S, price: Price) -> Result<Self> {
        let mut plan = Plan {
            base: AggregateRoot::new(id),
            events: Events::new(),
            name: name.into(),
            description: description.into(),
            price,
        };

        plan.events.record_event(PlanEvent::Created {
            id: plan.base().id().to_string(),
            name: plan.name().to_string(),
            description: plan.description().to_string(),
            price: plan.price().value(),
        });

        Ok(plan)
    }

    pub fn build(
        base: AggregateRoot<PlanId>,
        name: String,
        description: String,
        price: Price,
    ) -> Self {
        Plan {
            base,
            events: Events::new(),
            name,
            description,
            price,
        }
    }

    pub fn base(&self) -> &AggregateRoot<PlanId> {
        &self.base
    }

    pub fn events(&self) -> &Events<PlanEvent> {
        &self.events
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn price(&self) -> &Price {
        &self.price
    }

    pub fn set_name(&mut self, name: String) -> Result<()> {
        self.name = name;

        self.events.record_event(PlanEvent::NameChanged {
            id: self.base().id().to_string(),
            name: self.name.clone(),
        });

        Ok(())
    }

    pub fn set_description(&mut self, description: String) -> Result<()> {
        self.description = description;

        self.events.record_event(PlanEvent::DescriptionChanged {
            id: self.base().id().to_string(),
            description: self.description.clone(),
        });

        Ok(())
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
