mod author;
mod category;
mod collection;
mod collection_service;
mod publication;
mod publication_service;
mod repository;
mod statistics;
pub use author::*;
pub use category::*;
pub use collection::*;
pub use collection_service::*;
pub use publication::*;
pub use publication_service::*;
pub use repository::*;
pub use statistics::*;

use common::event::{ApplyEvent, Event};
use common::model::{AggregateRoot, StringId};
use common::result::Result;
use shared::event::PublicationEvent;

pub type CatalogueId = StringId;

#[derive(Debug, Clone)]
pub struct Catalogue {
    base: AggregateRoot<CatalogueId, Event>,
    authors: Vec<Author>,
    publications: Vec<Publication>,
    collections: Vec<Collection>,
}

impl Catalogue {
    pub fn new(id: CatalogueId) -> Result<Self> {
        Ok(Catalogue {
            base: AggregateRoot::new(id),
            authors: Vec::new(),
            publications: Vec::new(),
            collections: Vec::new(),
        })
    }

    pub fn base(&self) -> &AggregateRoot<CatalogueId, Event> {
        &self.base
    }

    pub fn authors(&self) -> &[Author] {
        &self.authors
    }

    pub fn publications(&self) -> &[Publication] {
        &self.publications
    }

    // TODO: improve logic!
    pub fn add_publication(&mut self, publication: Publication) {
        self.publications.push(publication);
        self.base.update();
    }

    pub fn remove_publication(&mut self, id: &str) {
        self.publications
            .retain(|publication| publication.id() != id);
        self.base.update();
    }

    pub fn add_collection(&mut self, collection: Collection) {
        self.collections.push(collection);
        self.base.update();
    }

    pub fn remove_collection(&mut self, id: &str) {
        self.collections.retain(|collection| collection.id() != id);
        self.base.update();
    }
}

impl ApplyEvent<PublicationEvent> for Catalogue {
    fn apply(&self, event: PublicationEvent) -> Result<()> {
        match event {
            PublicationEvent::Published { .. } => {
                println!("{:?}", event);
            }
            _ => {}
        }

        Ok(())
    }
}
