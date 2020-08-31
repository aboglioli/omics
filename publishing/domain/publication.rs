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
            id: publication.base().id().to_string(),
            author_id: publication.author_id().to_string(),
            name: publication.header().name().to_string(),
            synopsis: publication.header().synopsis().to_string(),
            category_id: publication.header().category_id().to_string(),
            tags: publication
                .header()
                .tags()
                .iter()
                .map(|t| t.name().to_string())
                .collect(),
            cover: publication.header().cover().url().to_string(),
        });

        Ok(publication)
    }

    pub fn build(
        base: AggregateRoot<PublicationId, PublicationEvent>,
        author_id: AuthorId,
        header: Header,

        pages: Vec<Page>,
        contract: bool,
        statistics: Statistics,

        status_history: StatusHistory<Status>,
    ) -> Self {
        Publication {
            base,
            author_id,
            header,
            pages,
            contract,
            statistics,
            status_history,
        }
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

    pub fn statistics_mut(&mut self) -> &mut Statistics {
        &mut self.statistics
    }

    pub fn status_history(&self) -> &StatusHistory<Status> {
        &self.status_history
    }

    pub fn is_published(&self) -> bool {
        self.base.deleted_at().is_none()
            && matches!(self.status_history().current(), Status::Published { .. })
    }

    pub fn set_header(&mut self, header: Header, author_id: &AuthorId) -> Result<()> {
        if self.author_id() != author_id {
            return Err(Error::new("publication", "not_owner"));
        }

        self.header = header;

        self.make_draft()?;

        self.base.record_event(PublicationEvent::HeaderUpdated {
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

    pub fn set_pages(&mut self, pages: Vec<Page>, author_id: &AuthorId) -> Result<()> {
        if self.author_id() != author_id {
            return Err(Error::new("publication", "not_owner"));
        }

        self.pages = pages;

        self.make_draft()?;

        self.base.record_event(PublicationEvent::PagesUpdated {
            id: self.base().id().to_string(),
            pages_count: self.pages().len(),
        });

        Ok(())
    }

    pub fn view(&mut self, reader: &Reader, unique: bool) -> Result<View> {
        if !self.is_published() {
            return Err(Error::new("publication", "not_published"));
        }

        self.statistics.add_view(unique);

        self.base.record_event(PublicationEvent::Viewed {
            reader_id: reader.base().id().to_string(),
            publication_id: self.base().id().to_string(),
            unique,
        });

        self.base.record_event(PublicationEvent::StatisticsUpdated {
            id: self.base().id().to_string(),
            views: self.statistics().views(),
            unique_views: self.statistics().unique_views(),
            readings: self.statistics().readings(),
            likes: self.statistics().likes(),
            reviews: self.statistics().reviews(),
            stars: self.statistics().stars(),
        });

        Ok(View::new(
            reader.base().id().clone(),
            self.base().id().clone(),
            unique,
        )?)
    }

    pub fn read(&mut self, reader: &Reader) -> Result<Reading> {
        if !self.is_published() {
            return Err(Error::new("publication", "not_published"));
        }

        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.statistics.add_reading();

        self.base.record_event(PublicationEvent::Read {
            reader_id: reader.base().id().to_string(),
            publication_id: self.base().id().to_string(),
        });

        self.base.record_event(PublicationEvent::StatisticsUpdated {
            id: self.base().id().to_string(),
            views: self.statistics().views(),
            unique_views: self.statistics().unique_views(),
            readings: self.statistics().readings(),
            likes: self.statistics().likes(),
            reviews: self.statistics().reviews(),
            stars: self.statistics().stars(),
        });

        Ok(Reading::new(
            reader.base().id().clone(),
            self.base().id().clone(),
        )?)
    }

    pub fn like(&mut self, reader: &Reader) -> Result<Like> {
        if !self.is_published() {
            return Err(Error::new("publication", "not_published"));
        }

        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.statistics.add_like();

        self.base.record_event(PublicationEvent::Liked {
            reader_id: reader.base().id().to_string(),
            publication_id: self.base().id().to_string(),
        });

        self.base.record_event(PublicationEvent::StatisticsUpdated {
            id: self.base().id().to_string(),
            views: self.statistics().views(),
            unique_views: self.statistics().unique_views(),
            readings: self.statistics().readings(),
            likes: self.statistics().likes(),
            reviews: self.statistics().reviews(),
            stars: self.statistics().stars(),
        });

        Ok(Like::new(
            reader.base().id().clone(),
            self.base().id().clone(),
        )?)
    }

    pub fn unlike(&mut self, reader: &Reader) -> Result<()> {
        if !self.is_published() {
            return Err(Error::new("publication", "not_published"));
        }

        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.statistics.remove_like();

        self.base.record_event(PublicationEvent::Unliked {
            reader_id: reader.base().id().to_string(),
            publication_id: self.base().id().to_string(),
        });

        self.base.record_event(PublicationEvent::StatisticsUpdated {
            id: self.base().id().to_string(),
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
        if !self.is_published() {
            return Err(Error::new("publication", "not_published"));
        }

        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.statistics.add_review(&stars);

        self.base.record_event(PublicationEvent::Reviewed {
            reader_id: reader.base().id().to_string(),
            publication_id: self.base().id().to_string(),
            stars: stars.value(),
            comment: comment.to_string(),
        });

        self.base.record_event(PublicationEvent::StatisticsUpdated {
            id: self.base().id().to_string(),
            views: self.statistics().views(),
            unique_views: self.statistics().unique_views(),
            readings: self.statistics().readings(),
            likes: self.statistics().likes(),
            reviews: self.statistics().reviews(),
            stars: self.statistics().stars(),
        });

        Ok(Review::new(
            reader.base().id().clone(),
            self.base().id().clone(),
            stars,
            comment,
        )?)
    }

    pub fn delete_review(&mut self, reader: &Reader, stars: &Stars) -> Result<()> {
        if !self.is_published() {
            return Err(Error::new("publication", "not_published"));
        }

        if self.has_contract() && !reader.is_subscribed() {
            return Err(Error::new("reader", "not_subscribed"));
        }

        self.statistics.remove_review(stars);

        self.base.record_event(PublicationEvent::ReviewDeleted {
            reader_id: reader.base().id().to_string(),
            publication_id: self.base().id().to_string(),
        });

        self.base.record_event(PublicationEvent::StatisticsUpdated {
            id: self.base().id().to_string(),
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
        if !self.is_published() {
            return Err(Error::new("publication", "not_published"));
        }

        if self.has_contract() {
            return Err(Error::new("publication", "already_has_a_contract"));
        }

        self.contract = true;

        self.base.record_event(PublicationEvent::ContractAdded {
            id: self.base().id().to_string(),
        });

        Ok(())
    }

    pub fn remove_contract(&mut self) -> Result<()> {
        if !self.has_contract() {
            return Err(Error::new("publication", "does_not_have_a_contract"));
        }

        self.contract = false;

        self.base.record_event(PublicationEvent::ContractAdded {
            id: self.base().id().to_string(),
        });

        Ok(())
    }

    pub fn make_draft(&mut self) -> Result<()> {
        if !matches!(self.status_history().current(), Status::Draft) {
            let draft = self.status_history.current().draft()?;
            self.status_history.add_status(draft);

            self.base.record_event(PublicationEvent::ChangedToDraft {
                id: self.base().id().to_string(),
            });
        }

        Ok(())
    }

    pub fn publish(&mut self, author: &Author) -> Result<()> {
        if author.base().id() != self.author_id() {
            return Err(Error::new("publication", "invalid_author"));
        }

        let waiting_approval = self.status_history.current().publish()?;

        if self.pages.is_empty() {
            return Err(Error::new("publication", "does_not_have_pages"));
        }

        for page in self.pages().iter() {
            if page.images().is_empty() {
                return Err(Error::new("publication", "empty_page")
                    .add_context("page", &page.number().to_string()));
            }
        }

        self.status_history.add_status(waiting_approval);

        self.base.record_event(PublicationEvent::ApprovalWaited {
            id: self.base().id().to_string(),
        });

        Ok(())
    }

    pub fn approve(&mut self, content_manager: &ContentManager) -> Result<()> {
        let published = self
            .status_history
            .current()
            .approve(content_manager.base().id().clone())?;
        self.status_history.add_status(published);

        self.base.record_event(PublicationEvent::Published {
            id: self.base().id().to_string(),
            author_id: self.author_id().to_string(),
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
            pages_count: self.pages().len(),
        });

        Ok(())
    }

    pub fn reject(&mut self, content_manager: &ContentManager) -> Result<()> {
        let rejected = self
            .status_history()
            .current()
            .reject(content_manager.base().id().clone())?;
        self.status_history.add_status(rejected);

        self.base.record_event(PublicationEvent::Rejected {
            id: self.base().id().to_string(),
        });

        Ok(())
    }

    pub fn delete(&mut self, author_id: &AuthorId) -> Result<()> {
        if self.author_id() != author_id {
            return Err(Error::new("publication", "not_owner"));
        }

        self.base.delete();

        self.base.record_event(PublicationEvent::Deleted {
            id: self.base().id().to_string(),
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
            publication.status_history().current(),
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
            publication.status_history().current(),
            Status::WaitingApproval
        ));

        assert!(publication.approve(&cm1).is_ok());
        assert!(matches!(publication.status_history().current(), Status::Published { .. }));
        assert!(publication.publish(&author).is_err());

        assert!(publication.make_draft().is_ok());
        assert!(matches!(
            publication.status_history().current(),
            Status::Draft
        ));
        assert!(publication.publish(&author).is_ok());

        assert!(publication.reject(&cm1).is_ok());
        assert!(matches!(publication.status_history().current(), Status::Rejected { .. }));
        assert!(publication.publish(&author).is_err());

        assert!(publication.make_draft().is_ok());
        assert!(matches!(
            publication.status_history().current(),
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
