mod name;
mod synopsis;
pub use name::*;
pub use synopsis::*;

use common::model::{AggregateRoot, StringId};
use common::result::Result;
use shared::domain::event::CollectionEvent;

pub type CollectionId = StringId;

pub struct Collection {
    base: AggregateRoot<CollectionId, CollectionEvent>,
    name: Name,
    synopsis: Synopsis,
}

impl Collection {
    pub fn new(id: CollectionId, name: Name, synopsis: Synopsis) -> Result<Self> {
        Ok(Collection {
            base: AggregateRoot::new(id),
            name,
            synopsis,
        })
    }

    pub fn base(&self) -> &AggregateRoot<CollectionId, CollectionEvent> {
        &self.base
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn synopsis(&self) -> &Synopsis {
        &self.synopsis
    }
}
