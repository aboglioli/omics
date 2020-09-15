mod name;
mod repository;
pub use name::*;
pub use repository::*;

use common::model::{AggregateRoot, Events, StringId};
use common::result::Result;
use shared::event::CategoryEvent;

pub type CategoryId = StringId;

#[derive(Debug, Clone)]
pub struct Category {
    base: AggregateRoot<CategoryId>,
    events: Events<CategoryEvent>,
    name: Name,
}

impl Category {
    pub fn new(id: CategoryId, name: Name) -> Result<Self> {
        let mut category = Category {
            base: AggregateRoot::new(id),
            events: Events::new(),
            name,
        };

        category.events.record_event(CategoryEvent::Created {
            id: category.base().id().to_string(),
            name: category.name().to_string(),
        });

        Ok(category)
    }

    pub fn build(base: AggregateRoot<CategoryId>, name: Name) -> Self {
        Category {
            base,
            events: Events::new(),
            name,
        }
    }

    pub fn base(&self) -> &AggregateRoot<CategoryId> {
        &self.base
    }

    pub fn events(&self) -> &Events<CategoryEvent> {
        &self.events
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn set_name(&mut self, name: Name) -> Result<()> {
        self.name = name;
        self.base.update();

        self.events.record_event(CategoryEvent::Updated {
            id: self.base().id().to_string(),
            name: self.name().to_string(),
        });

        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.base.delete();

        self.events.record_event(CategoryEvent::Deleted {
            id: self.base().id().to_string(),
        });

        Ok(())
    }
}
