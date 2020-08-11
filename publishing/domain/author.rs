mod author_repository;
pub use author_repository::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type AuthorId = StringId;

pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: &str) -> Result<Self> {
        Ok(Name {
            name: name.to_owned(),
        })
    }

    pub fn value(&self) -> &str {
        &self.name
    }
}

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
