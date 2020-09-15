use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::interaction::ReaderPublicationId;

use crate::domain::interaction::{Comment, Stars};

#[derive(Debug, Clone)]
pub struct Review {
    base: AggregateRoot<ReaderPublicationId>,
    stars: Stars,
    comment: Comment,
}

impl Review {
    pub fn new(id: ReaderPublicationId, stars: Stars, comment: Comment) -> Result<Self> {
        Ok(Review {
            base: AggregateRoot::new(id),
            stars,
            comment,
        })
    }

    pub fn base(&self) -> &AggregateRoot<ReaderPublicationId> {
        &self.base
    }

    pub fn stars(&self) -> &Stars {
        &self.stars
    }

    pub fn comment(&self) -> &Comment {
        &self.comment
    }
}
