mod repository;
mod statistics;
pub use repository::*;
pub use statistics::*;

use common::model::{AggregateRoot, StringId};
use common::result::Result;

use crate::domain::user::User;

pub type PublicationId = StringId;

#[derive(Debug, Clone)]
pub struct Publication {
    base: AggregateRoot<PublicationId>,
    author: User,
    statistics: Statistics,
}

impl Publication {
    pub fn new(id: PublicationId, author: User, statistics: Statistics) -> Result<Self> {
        Ok(Publication {
            base: AggregateRoot::new(id),
            author,
            statistics,
        })
    }

    pub fn base(&self) -> &AggregateRoot<PublicationId> {
        &self.base
    }

    pub fn author(&self) -> &User {
        &self.author
    }

    pub fn statistics(&self) -> &Statistics {
        &self.statistics
    }
}
