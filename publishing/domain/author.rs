mod repository;
pub use repository::*;

use common::error::Error;
use common::model::{AggregateRoot, StringId};
use common::result::Result;
use shared::event::AuthorEvent;

use crate::domain::reader::Reader;

pub type AuthorId = StringId;

#[derive(Debug, Clone)]
pub struct Author {
    base: AggregateRoot<AuthorId, AuthorEvent>,
    username: String,
    name: String,
    lastname: String,
}

impl Author {
    pub fn new<S: Into<String>>(id: AuthorId, username: S, name: S, lastname: S) -> Result<Self> {
        Ok(Author {
            base: AggregateRoot::new(id),
            username: username.into(),
            name: name.into(),
            lastname: lastname.into(),
        })
    }

    pub fn base(&self) -> &AggregateRoot<AuthorId, AuthorEvent> {
        &self.base
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lastname(&self) -> &str {
        &self.lastname
    }

    pub fn follow(&mut self, reader: &Reader) -> Result<()> {
        if self.base().id() == reader.base().id() {
            return Err(Error::new("author", "cannot_follow_itself"));
        }

        self.base.record_event(AuthorEvent::Followed {
            author_id: self.base().id().to_string(),
            reader_id: reader.base().id().to_string(),
        });

        Ok(())
    }
}
