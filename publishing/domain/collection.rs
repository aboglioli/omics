mod item;
mod repository;
pub use item::*;
pub use repository::*;

use common::error::Error;
use common::model::{AggregateRoot, StringId};
use common::result::Result;
use shared::event::CollectionEvent;

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
            id: collection.base().id().to_string(),
            author_id: collection.author_id().to_string(),
            name: collection.header().name().to_string(),
            synopsis: collection.header().synopsis().to_string(),
            category_id: collection.header().category_id().to_string(),
            tags: collection
                .header()
                .tags()
                .iter()
                .map(|t| t.name().to_string())
                .collect(),
            cover: collection.header().cover().url().to_string(),
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

    pub fn set_header(&mut self, header: Header, author_id: &AuthorId) -> Result<()> {
        if self.author_id() != author_id {
            return Err(Error::new("collection", "not_owner"));
        }

        self.header = header;

        self.base.record_event(CollectionEvent::HeaderUpdated {
            id: self.base().id().to_string(),
            name: self.header().name().to_string(),
            synopsis: self.header().synopsis().to_string(),
            category_id: self.header().category_id().to_string(),
            tags: self
                .header()
                .tags()
                .iter()
                .map(|t| t.name().to_string())
                .collect(),
            cover: self.header().cover().url().to_string(),
        });

        Ok(())
    }

    pub fn add_item(&mut self, publication: &Publication, author_id: &AuthorId) -> Result<()> {
        if self.author_id() != author_id {
            return Err(Error::new("collection", "not_owner"));
        }

        if !publication.is_published() {
            return Err(Error::new("collection", "publication_is_not_published"));
        }

        for item in self.items() {
            if item.publication_id() == publication.base().id() {
                return Err(Error::new("collection", "publication_exists"));
            }
        }

        let item = Item::new(publication.base().id().clone())?;
        self.items.push(item);

        self.base.record_event(CollectionEvent::PublicationAdded {
            id: self.base().id().to_string(),
            publication_id: publication.base().id().to_string(),
        });

        Ok(())
    }

    pub fn remove_item(
        &mut self,
        publication_id: &PublicationId,
        author_id: &AuthorId,
    ) -> Result<()> {
        if self.author_id() != author_id {
            return Err(Error::new("collection", "not_owner"));
        }

        self.items
            .retain(|item| item.publication_id() != publication_id);

        self.base.record_event(CollectionEvent::PublicationRemoved {
            id: self.base().id().to_string(),
            publication_id: publication_id.to_string(),
        });

        Ok(())
    }

    pub fn delete(&mut self, author_id: &AuthorId) -> Result<()> {
        if self.author_id() != author_id {
            return Err(Error::new("collection", "not_owner"));
        }

        self.base.delete();

        self.base.record_event(CollectionEvent::Deleted {
            id: self.base().id().to_string(),
        });

        Ok(())
    }
}
