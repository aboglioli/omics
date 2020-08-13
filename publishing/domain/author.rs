mod repository;
pub use repository::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type AuthorId = StringId;

#[derive(Debug, Clone)]
pub struct Author {
    base: AggregateRoot<AuthorId, Event>,
}

impl Author {
    pub fn new(id: AuthorId) -> Result<Self> {
        Ok(Author {
            base: AggregateRoot::new(id),
        })
    }

    pub fn base(&self) -> &AggregateRoot<AuthorId, Event> {
        &self.base
    }
}
