mod repository;
pub use repository::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type AuthorId = StringId;

#[derive(Debug, Clone)]
pub struct Author {
    base: AggregateRoot<AuthorId, Event>,
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

    pub fn base(&self) -> &AggregateRoot<AuthorId, Event> {
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
}
