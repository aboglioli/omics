mod frame;
mod header;
mod image;
mod name;
mod page;
mod repository;
mod statistics;
mod statistics_service;
mod status;
mod synopsis;
mod tag;
pub use frame::*;
pub use header::*;
pub use image::*;
pub use name::*;
pub use page::*;
pub use repository::*;
pub use statistics::*;
pub use statistics_service::*;
pub use status::*;
pub use synopsis::*;
pub use tag::*;

use common::error::Error;
use common::model::{AggregateRoot, StatusHistory, StringId};
use common::result::Result;
use shared::event::PublicationEvent;

use crate::domain::author::{Author, AuthorId};
use crate::domain::content_manager::ContentManager;
use crate::domain::interaction::{Comment, Like, Reading, Review, Stars, View};
use crate::domain::reader::Reader;

pub type PublicationId = StringId;

#[derive(Debug, Clone)]
pub struct Publication {
    base: AggregateRoot<PublicationId, PublicationEvent>,
    author_id: AuthorId,
    header: Header,

    pages: Vec<Page>,
    contract: bool,
    statistics: Statistics,

    status_history: StatusHistory<Status>,
}

impl Publication {
    pub fn new(id: PublicationId, author_id: AuthorId, header: Header) -> Result<Self> {
        let mut publication = Publication {
            base: AggregateRoot::new(id),
            author_id,
            header,
            pages: Vec::new(),
            contract: false,
            statistics: Statistics::default(),
            status_history: StatusHistory::new(Status::Draft),
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

    pub fn statistics(&self) -> &Statistics {
        &self.statistics
    }

    pub fn status_history(&self) -> &StatusHistory<Status> {
        &self.status_history
    }

    pub fn is_published(&self) -> bool {
        self.base.deleted_at().is_none()
            && matches!(self.status_history().current().status(), Status::Published { .. })
    }

    pub fn set_header(&mut self, header: Header) -> Result<()> {
        self.header = header;

        self.make_draft()?;

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

        self.make_draft()?;

        self.base.record_event(PublicationEvent::PagesUpdated {
            id: self.base().id().value().to_owned(),
            pages_count: self.pages().len(),
        });

        Ok(())
    }

    pub fn view(&mut self, reader: &Reader, unique: bool) -> Result<View> {
        if !matches!(self.status_history().current().status(), Status::Published { .. }) {
            return Err(Error::new("publication", "not_published"));
        }

        self.statistics.add_view(unique);

        self.base.record_event(PublicationEvent::Viewed {
            reader_id: reader.base().id().value().to_owned(),
            publication_id: self.base().id().value().to_owned(),
            unique,
        });

        self.base.record_event(PublicationEvent::StatisticsUpdated {
            id: self.base().id().value().to_owned(),
            views: self.statistics().views(),
            unique_views: self.statistics().unique_views(),
            readings: self.statistics().readings(),
            likes: self.statistics().likes(),
            reviews: self.statistics().reviews(),
            stars: self.statistics().stars(),
        });

        Ok(View::new(reader.base().id(), self.base().id(), unique)?)
    }

    pub fn read(&mut self, reader: &Reader) -> Result<Reading> {
        if !matches!(self.status_history().current().status(), Status::Published { .. }) {
            return Err(Error::new("publication", "not_published"));
        }

        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.statistics.add_reading();

        self.base.record_event(PublicationEvent::Read {
            reader_id: reader.base().id().value().to_owned(),
            publication_id: self.base().id().value().to_owned(),
        });

        self.base.record_event(PublicationEvent::StatisticsUpdated {
            id: self.base().id().value().to_owned(),
            views: self.statistics().views(),
            unique_views: self.statistics().unique_views(),
            readings: self.statistics().readings(),
            likes: self.statistics().likes(),
            reviews: self.statistics().reviews(),
            stars: self.statistics().stars(),
        });

        Ok(Reading::new(reader.base().id(), self.base().id())?)
    }

    pub fn like(&mut self, reader: &Reader) -> Result<Like> {
        if !matches!(self.status_history().current().status(), Status::Published { .. }) {
            return Err(Error::new("publication", "not_published"));
        }

        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.statistics.add_like();

        self.base.record_event(PublicationEvent::Liked {
            reader_id: reader.base().id().value().to_owned(),
            publication_id: self.base().id().value().to_owned(),
        });

        self.base.record_event(PublicationEvent::StatisticsUpdated {
            id: self.base().id().value().to_owned(),
            views: self.statistics().views(),
            unique_views: self.statistics().unique_views(),
            readings: self.statistics().readings(),
            likes: self.statistics().likes(),
            reviews: self.statistics().reviews(),
            stars: self.statistics().stars(),
        });

        Ok(Like::new(reader.base().id(), self.base().id())?)
    }

    pub fn unlike(&mut self, reader: &Reader) -> Result<()> {
        if !matches!(self.status_history().current().status(), Status::Published { .. }) {
            return Err(Error::new("publication", "not_published"));
        }

        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.statistics.remove_like();

        self.base.record_event(PublicationEvent::Unliked {
            reader_id: reader.base().id().value().to_owned(),
            publication_id: self.base().id().value().to_owned(),
        });

        self.base.record_event(PublicationEvent::StatisticsUpdated {
            id: self.base().id().value().to_owned(),
            views: self.statistics().views(),
            unique_views: self.statistics().unique_views(),
            readings: self.statistics().readings(),
            likes: self.statistics().likes(),
            reviews: self.statistics().reviews(),
            stars: self.statistics().stars(),
        });

        Ok(())
    }

    pub fn review(&mut self, reader: &Reader, stars: Stars, comment: Comment) -> Result<Review> {
        if !matches!(self.status_history().current().status(), Status::Published { .. }) {
            return Err(Error::new("publication", "not_published"));
        }

        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.statistics.add_review(&stars);

        self.base.record_event(PublicationEvent::Reviewed {
            reader_id: reader.base().id().value().to_owned(),
            publication_id: self.base().id().value().to_owned(),
            stars: stars.value(),
            comment: comment.value().to_owned(),
        });

        self.base.record_event(PublicationEvent::StatisticsUpdated {
            id: self.base().id().value().to_owned(),
            views: self.statistics().views(),
            unique_views: self.statistics().unique_views(),
            readings: self.statistics().readings(),
            likes: self.statistics().likes(),
            reviews: self.statistics().reviews(),
            stars: self.statistics().stars(),
        });

        Ok(Review::new(
            reader.base().id(),
            self.base().id(),
            stars,
            comment,
        )?)
    }

    pub fn delete_review(&mut self, reader: &Reader, stars: &Stars) -> Result<()> {
        if !matches!(self.status_history().current().status(), Status::Published { .. }) {
            return Err(Error::new("publication", "not_published"));
        }

        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.statistics.remove_review(stars);

        self.base.record_event(PublicationEvent::ReviewDeleted {
            reader_id: reader.base().id().value().to_owned(),
            publication_id: self.base().id().value().to_owned(),
        });

        self.base.record_event(PublicationEvent::StatisticsUpdated {
            id: self.base().id().value().to_owned(),
            views: self.statistics().views(),
            unique_views: self.statistics().unique_views(),
            readings: self.statistics().readings(),
            likes: self.statistics().likes(),
            reviews: self.statistics().reviews(),
            stars: self.statistics().stars(),
        });

        Ok(())
    }

    pub fn add_contract(&mut self) -> Result<()> {
        if !matches!(self.status_history().current().status(), Status::Published { .. }) {
            return Err(Error::new("publication", "not_published"));
        }

        if self.has_contract() {
            return Err(Error::new("publication", "already_has_a_contract"));
        }

        self.contract = true;

        self.base.record_event(PublicationEvent::ContractAdded {
            id: self.base().id().value().to_owned(),
        });

        Ok(())
    }

    pub fn remove_contract(&mut self) -> Result<()> {
        if !self.has_contract() {
            return Err(Error::new("publication", "does_not_have_a_contract"));
        }

        self.contract = false;

        self.base.record_event(PublicationEvent::ContractAdded {
            id: self.base().id().value().to_owned(),
        });

        Ok(())
    }

    pub fn make_draft(&mut self) -> Result<()> {
        if !matches!(self.status_history().current().status(), Status::Draft) {
            self.status_history.add_status(Status::Draft);

            self.base.record_event(PublicationEvent::ChangedToDraft {
                id: self.base().id().value().to_owned(),
            });
        }

        Ok(())
    }

    pub fn publish(&mut self, author: &Author) -> Result<()> {
        if &author.base().id() != self.author_id() {
            return Err(Error::new("publication", "invalid_author"));
        }

        if !matches!(self.status_history().current().status(), Status::Draft) {
            return Err(Error::new("publication", "not_a_draft"));
        }

        if self.pages.len() == 0 {
            return Err(Error::new("publication", "does_not_have_pages"));
        }

        for page in self.pages().iter() {
            if page.images().len() == 0 {
                return Err(Error::new("publication", "empty_page")
                    .add_context("page", &page.number().to_string())
                    .build());
            }
        }

        self.status_history.add_status(Status::WaitingApproval);

        self.base.record_event(PublicationEvent::ApprovalWaited {
            id: self.base().id().value().to_owned(),
        });

        Ok(())
    }

    pub fn approve(&mut self, content_manager: &ContentManager) -> Result<()> {
        if !matches!(
            self.status_history().current().status(),
            Status::WaitingApproval
        ) {
            return Err(Error::new("publication", "not_waiting_approval"));
        }

        self.status_history.add_status(Status::Published {
            admin_id: content_manager.base().id(),
        });

        self.base.record_event(PublicationEvent::Published {
            id: self.base().id().value().to_owned(),
        });

        Ok(())
    }

    pub fn reject(&mut self, content_manager: &ContentManager) -> Result<()> {
        if !matches!(
            self.status_history().current().status(),
            Status::WaitingApproval
        ) {
            return Err(Error::new("publication", "not_waiting_approval"));
        }

        self.status_history.add_status(Status::Rejected {
            admin_id: content_manager.base().id(),
        });

        self.base.record_event(PublicationEvent::Rejected {
            id: self.base().id().value().to_owned(),
        });

        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.base.delete();

        self.base.record_event(PublicationEvent::Deleted {
            id: self.base().id().value().to_owned(),
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[test]
    fn create() {
        let publication = mocks::publication1();

        assert_eq!(publication.base().id().value(), "#publication01");
        assert_eq!(publication.header().name().value(), "Publication 01");
        assert_eq!(publication.header().synopsis().value(), "Synopsis...");
        assert_eq!(publication.header().category_id().value(), "#category01");
        assert_eq!(publication.header().tags().len(), 2);
        assert!(publication.base().events().unwrap().len() > 0);
        assert!(matches!(
            publication.status_history().current().status(),
            Status::Draft
        ));
    }

    #[test]
    fn status() {
        let mut publication = mocks::publication1();
        let cm1 = mocks::content_manager1();
        let author = mocks::author1();

        assert!(publication.make_draft().is_ok());
        assert!(publication.make_draft().is_ok());

        assert!(publication.approve(&cm1).is_err());
        assert!(publication.reject(&cm1).is_err());

        assert!(publication.publish(&author).is_ok());
        assert!(matches!(
            publication.status_history().current().status(),
            Status::WaitingApproval
        ));

        assert!(publication.approve(&cm1).is_ok());
        assert!(
            matches!(publication.status_history().current().status(), Status::Published { .. })
        );
        assert!(publication.publish(&author).is_err());

        assert!(publication.make_draft().is_ok());
        assert!(matches!(
            publication.status_history().current().status(),
            Status::Draft
        ));
        assert!(publication.publish(&author).is_ok());

        assert!(publication.reject(&cm1).is_ok());
        assert!(matches!(publication.status_history().current().status(), Status::Rejected { .. }));
        assert!(publication.publish(&author).is_err());

        assert!(publication.make_draft().is_ok());
        assert!(matches!(
            publication.status_history().current().status(),
            Status::Draft
        ));
    }

    #[test]
    fn interaction_with_draft_publication() {
        let mut publication = mocks::publication1();
        let reader = mocks::reader1();
        let comment = Comment::new("comment").unwrap();
        let stars = Stars::new(5).unwrap();

        assert!(publication.like(&reader).is_err());
        assert!(publication.unlike(&reader).is_err());
        assert!(publication
            .review(&reader, stars.clone(), comment.clone())
            .is_err());
        assert!(publication.delete_review(&reader, &stars).is_err());
        assert!(publication.add_contract().is_err());
    }

    #[test]
    fn interaction_with_published_publication() {
        let mut publication = mocks::published_publication1();
        let reader = mocks::reader1();
        let comment = Comment::new("comment").unwrap();
        let stars = Stars::new(5).unwrap();

        assert!(publication.like(&reader).is_ok());
        assert!(publication.unlike(&reader).is_ok());
        assert!(publication
            .review(&reader, stars.clone(), comment.clone())
            .is_ok());
        assert!(publication.delete_review(&reader, &stars).is_ok());
        assert!(publication.add_contract().is_ok());
        assert!(publication.remove_contract().is_ok());

        // First events: Created, PagesUpdated, ApprovalWaited (publish), Published (approve)
        assert!(publication.base().events().unwrap().len() > 0);
    }
}
