mod repository;
pub use repository::*;

use common::error::Error;
use common::model::{AggregateRoot, Events, StringId};
use common::result::Result;
use shared::event::AuthorEvent;

use crate::domain::reader::Reader;

pub type AuthorId = StringId;

#[derive(Debug, Clone)]
pub struct Author {
    base: AggregateRoot<AuthorId>,
    events: Events<AuthorEvent>,
    followers: u32,
}

impl Author {
    pub fn new(id: AuthorId) -> Result<Self> {
        Ok(Author {
            base: AggregateRoot::new(id),
            events: Events::new(),
            followers: 0,
        })
    }

    pub fn base(&self) -> &AggregateRoot<AuthorId> {
        &self.base
    }

    pub fn events(&self) -> &Events<AuthorEvent> {
        &self.events
    }

    pub fn followers(&self) -> u32 {
        self.followers
    }

    pub fn follow(&mut self, reader: &Reader) -> Result<()> {
        if self.base().id() == reader.base().id() {
            return Err(Error::new("author", "cannot_follow_itself"));
        }

        self.followers += 1;
        self.base.update();

        self.events.record_event(AuthorEvent::Followed {
            author_id: self.base().id().to_string(),
            reader_id: reader.base().id().to_string(),
        });

        Ok(())
    }

    pub fn unfollow(&mut self, reader: &Reader) -> Result<()> {
        if self.base().id() == reader.base().id() {
            return Err(Error::new("author", "cannot_unfollow_itself"));
        }

        if self.followers == 0 {
            return Err(Error::new("author", "does_not_have_followers"));
        }

        self.followers -= 1;
        self.base.update();

        self.events.record_event(AuthorEvent::Unfollowed {
            author_id: self.base().id().to_string(),
            reader_id: reader.base().id().to_string(),
        });

        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.base.delete();
        Ok(())
    }
}
