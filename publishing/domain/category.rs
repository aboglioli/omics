mod name;
mod repository;
pub use name::*;
pub use repository::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type CategoryId = StringId;

#[derive(Debug, Clone)]
pub struct Category {
    base: AggregateRoot<CategoryId, Event>,
    name: Name,
}

impl Category {
    pub fn new(id: CategoryId, name: Name) -> Result<Self> {
        Ok(Category {
            base: AggregateRoot::new(id),
            name,
        })
    }

    pub fn base(&self) -> &AggregateRoot<CategoryId, Event> {
        &self.base
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn set_name(&mut self, name: Name) -> Result<()> {
        self.name = name;
        self.base.update();
        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.base.delete();
        Ok(())
    }
}
