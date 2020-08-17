mod repository;
mod statistics;
pub use repository::*;
pub use statistics::*;

use common::model::StringId;
use common::result::Result;

pub type PublicationId = StringId;

use crate::domain::user::User;

#[derive(Debug, Clone)]
pub struct Publication {
    id: PublicationId,
    author: User,
    statistics: Statistics,
}

impl Publication {
    pub fn new(id: PublicationId, author: User, statistics: Statistics) -> Result<Self> {
        Ok(Publication {
            id,
            author,
            statistics,
        })
    }

    pub fn id(&self) -> &PublicationId {
        &self.id
    }

    pub fn author(&self) -> &User {
        &self.author
    }

    pub fn statistics(&self) -> &Statistics {
        &self.statistics
    }
}
