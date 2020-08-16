mod author;
mod category;
mod publication;
mod publication_service;
mod repository;
mod statistics;
pub use author::*;
pub use category::*;
pub use publication::*;
pub use publication_service::*;
pub use repository::*;
pub use statistics::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type CatalogueId = StringId;

#[derive(Debug, Clone)]
pub struct Catalogue {
    base: AggregateRoot<CatalogueId, Event>,
    authors: Vec<Author>,
    publications: Vec<Publication>,
}

// TODO: implement!
impl Catalogue {
    pub fn new(id: CatalogueId) -> Result<Self> {
        Ok(Catalogue {
            base: AggregateRoot::new(id),
            authors: Vec::new(),
            publications: Vec::new(),
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

    pub fn add_publication(&mut self, publication: Publication) {
        self.publications.push(publication);
        self.base.update();
    }

    pub fn remove_publication(&mut self, _id: &str) {
        self.base.update();
    }
}
