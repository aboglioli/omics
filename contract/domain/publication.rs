mod repository;
pub use repository::*;

use common::model::StringId;
use common::result::Result;

pub type PublicationId = StringId;

use crate::domain::user::User;

#[derive(Debug, Clone)]
pub struct Publication {
    id: PublicationId,
    author: User,
}

impl Publication {
    pub fn new(id: PublicationId, author: User) -> Result<Self> {
        Ok(Publication { id, author })
    }

    pub fn id(&self) -> &PublicationId {
        &self.id
    }

    pub fn author(&self) -> &User {
        &self.author
    }
}
