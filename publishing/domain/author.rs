mod repository;
pub use repository::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type AuthorId = StringId;

#[derive(Debug, Clone)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new<S: Into<String>>(name: S) -> Result<Self> {
        Ok(Name { name: name.into() })
    }

    pub fn value(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct Author {
    base: AggregateRoot<AuthorId, Event>,
    name: Name,
}

impl Author {
    pub fn new(id: AuthorId, name: Name) -> Result<Self> {
        Ok(Author {
            base: AggregateRoot::new(id),
            name,
        })
    }

    pub fn base(&self) -> &AggregateRoot<AuthorId, Event> {
        &self.base
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}
