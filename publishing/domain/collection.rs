mod name;
mod publication;
mod repository;
mod synopsis;
pub use name::*;
pub use publication::*;
pub use repository::*;
pub use synopsis::*;

use common::model::{AggregateRoot, StringId};
use common::result::Result;
use shared::domain::event::CollectionEvent;

pub type CollectionId = StringId;

#[derive(Debug, Clone)]
pub struct Collection {
    base: AggregateRoot<CollectionId, CollectionEvent>,
    name: Name,
    synopsis: Synopsis,
    publications: Vec<Publication>,
}

impl Collection {
    pub fn new(id: CollectionId, name: Name, synopsis: Synopsis) -> Result<Self> {
        Ok(Collection {
            base: AggregateRoot::new(id),
            name,
            synopsis,
            publications: Vec::new(),
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

    pub fn set_publications(&mut self, publications: Vec<Publication>) -> Result<()> {
        self.publications = publications;

        Ok(())
    }
}
