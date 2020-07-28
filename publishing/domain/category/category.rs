use common::error::Error;
use common::model::{AggregateRoot, DefaultEvent};

#[derive(Debug, Clone)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: &str) -> Result<Name, Error> {
        Ok(Name {
            name: name.to_owned(),
        })
    }

    pub fn value(&self) -> &str {
        &self.name
    }
}

pub type CategoryId = String;

#[derive(Debug, Clone)]
pub struct Category {
    base: AggregateRoot<CategoryId, DefaultEvent>,
    name: Name,
}

impl Category {
    pub fn new(id: CategoryId, name: Name) -> Result<Category, Error> {
        Ok(Category {
            base: AggregateRoot::new(id),
            name,
        })
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}
