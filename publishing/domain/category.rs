mod category_repository;
pub use category_repository::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: &str) -> Result<Name> {
        Ok(Name {
            name: name.to_owned(),
        })
    }

    pub fn value(&self) -> &str {
        &self.name
    }
}

pub type CategoryId = StringId;

#[derive(Debug, Clone)]
pub struct Category {
    base: AggregateRoot<CategoryId, Event>,
    name: Name,
}

impl Category {
    pub fn new(id: CategoryId, name: Name) -> Result<Category> {
        Ok(Category {
            base: AggregateRoot::new(id),
            name,
        })
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}
