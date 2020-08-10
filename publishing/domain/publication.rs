mod header;
mod name;
mod page;
mod publication_repository;
mod status;
mod synopsis;
mod tag;
pub use header::*;
pub use name::*;
pub use page::*;
pub use publication_repository::*;
pub use status::*;
pub use synopsis::*;
pub use tag::*;

use common::error::Error;
use common::model::{AggregateRoot, StatusHistory, StringId};
use common::result::Result;
use shared::domain::event::PublicationEvent;

use crate::domain::author::AuthorId;
use crate::domain::interaction::Stars;
use crate::domain::reader::Reader;

pub type PublicationId = StringId;

pub struct Publication {
    base: AggregateRoot<PublicationId, PublicationEvent>,
    author_id: AuthorId,
    header: Header,

    pages: Vec<Page>,
    contract: bool,

    status_history: StatusHistory<PublicationStatus>,
}

impl Publication {
    pub fn new(id: PublicationId, author_id: AuthorId, header: Header) -> Result<Publication> {
        let mut publication = Publication {
            base: AggregateRoot::new(id),
            author_id,
            header,
            pages: Vec::new(),
            contract: false,
            status_history: StatusHistory::new(PublicationStatus::Draft),
        };

        publication.base.record_event(PublicationEvent::Created {
            id: publication.base().id().value().to_owned(),
            author_id: publication.author_id().value().to_owned(),
            name: publication.header().name().value().to_owned(),
            synopsis: publication.header().synopsis().value().to_owned(),
            category_id: publication.header().category_id().value().to_owned(),
            tags: publication
                .header()
                .tags()
                .iter()
                .map(|t| t.name().to_owned())
                .collect(),
            cover: publication.header().cover().url().to_owned(),
        });

        Ok(publication)
    }

    pub fn base(&self) -> &AggregateRoot<PublicationId, PublicationEvent> {
        &self.base
    }

    pub fn author_id(&self) -> &AuthorId {
        &self.author_id
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    pub fn has_contract(&self) -> bool {
        self.contract
    }

    pub fn status_history(&self) -> &StatusHistory<PublicationStatus> {
        &self.status_history
    }

    pub fn set_header(&mut self, header: Header) -> Result<()> {
        self.header = header;

        self.base.record_event(PublicationEvent::HeaderUpdated {
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

    pub fn set_pages(&mut self, pages: Vec<Page>) -> Result<()> {
        self.pages = pages;

        self.base.record_event(PublicationEvent::PagesUpdated {
            id: self.base().id().value().to_owned(),
            pages_count: self.pages().len(),
        });

        Ok(())
    }

    pub fn view(&mut self, reader: &Reader) -> Result<()> {
        self.base.record_event(PublicationEvent::Viewed {
            reader_id: reader.base().id().value().to_owned(),
            publication_id: self.base().id().value().to_owned(),
        });

        Ok(())
    }

    pub fn read(&mut self, reader: &Reader) -> Result<()> {
        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.base.record_event(PublicationEvent::Read {
            reader_id: reader.base().id().value().to_owned(),
            publication_id: self.base().id().value().to_owned(),
        });

        Ok(())
    }

    pub fn like(&mut self, reader: &Reader) -> Result<()> {
        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.base.record_event(PublicationEvent::Liked {
            reader_id: reader.base().id().value().to_owned(),
            publication_id: self.base().id().value().to_owned(),
        });

        Ok(())
    }

    pub fn unlike(&mut self, reader: &Reader) -> Result<()> {
        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.base.record_event(PublicationEvent::Unliked {
            reader_id: reader.base().id().value().to_owned(),
            publication_id: self.base().id().value().to_owned(),
        });

        Ok(())
    }

    pub fn review(&mut self, reader: &Reader, stars: &Stars) -> Result<()> {
        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.base.record_event(PublicationEvent::Reviewed {
            reader_id: reader.base().id().value().to_owned(),
            publication_id: self.base().id().value().to_owned(),
            stars: stars.value(),
        });

        Ok(())
    }

    pub fn delete_review(&mut self, reader: &Reader) -> Result<()> {
        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.base.record_event(PublicationEvent::ReviewDeleted {
            reader_id: reader.base().id().value().to_owned(),
            publication_id: self.base().id().value().to_owned(),
        });

        Ok(())
    }

    pub fn add_contract(&mut self) -> Result<()> {
        self.contract = true;
        Ok(())
    }

    pub fn remove_contract(&mut self) -> Result<()> {
        self.contract = false;
        Ok(())
    }
}
