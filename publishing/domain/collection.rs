mod item;
mod repository;
pub use item::*;
pub use repository::*;

use common::error::Error;
use common::model::{AggregateRoot, StringId};
use common::result::Result;
use domain::event::CollectionEvent;

use crate::domain::author::AuthorId;
use crate::domain::publication::{Header, Publication, PublicationId};

pub type CollectionId = StringId;

#[derive(Debug, Clone)]
pub struct Collection {
    base: AggregateRoot<CollectionId, CollectionEvent>,
    author_id: AuthorId,
    header: Header,

    items: Vec<Item>,
}

impl Collection {
    pub fn new(id: CollectionId, author_id: AuthorId, header: Header) -> Result<Self> {
        let mut collection = Collection {
            base: AggregateRoot::new(id),
            author_id,
            header,
            items: Vec::new(),
        };

        collection.base.record_event(CollectionEvent::Created {
            id: collection.base().id().value().to_owned(),
            author_id: collection.author_id().value().to_owned(),
            name: collection.header().name().value().to_owned(),
            synopsis: collection.header().synopsis().value().to_owned(),
            category_id: collection.header().category_id().value().to_owned(),
            tags: collection
                .header()
                .tags()
                .iter()
                .map(|t| t.name().to_owned())
                .collect(),
            cover: collection.header().cover().url().to_owned(),
        });

        Ok(collection)
    }

    pub fn base(&self) -> &AggregateRoot<CollectionId, CollectionEvent> {
        &self.base
    }

    pub fn author_id(&self) -> &AuthorId {
        &self.author_id
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn items(&self) -> &[Item] {
        &self.items
    }

    pub fn set_header(&mut self, header: Header) -> Result<()> {
        self.header = header;

        self.base.record_event(CollectionEvent::HeaderUpdated {
            id: self.base().id().value().to_owned(),
            name: self.header().name().value().to_owned(),
            synopsis: self.header().synopsis().value().to_owned(),
            category_id: self.header().category_id().value().to_owned(),
            tags: self
                .header()
                .tags()
                .iter()
                .map(|t| t.name().to_owned())
                .collect(),
            cover: self.header().cover().url().to_owned(),
        });

        Ok(())
    }

    pub fn add_item(&mut self, publication: &Publication) -> Result<()> {
        if !publication.is_published() {
            return Err(Error::new("collection", "publication_is_not_published"));
        }

        let item = Item::new(publication.base().id())?;
        self.items.push(item);

        self.base.record_event(CollectionEvent::PublicationAdded {
            id: self.base().id().value().to_owned(),
            publication_id: publication.base().id().value().to_owned(),
        });

        Ok(())
    }

    pub fn remove_item(&mut self, publication_id: &PublicationId) -> Result<()> {
        self.items
            .retain(|item| item.publication_id() != publication_id);

        self.base.record_event(CollectionEvent::PublicationRemoved {
            id: self.base().id().value().to_owned(),
            publication_id: publication_id.value().to_owned(),
        });

        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.base.delete();

        self.base.record_event(CollectionEvent::Deleted {
            id: self.base().id().value().to_owned(),
        });

        Ok(())
    }
}
