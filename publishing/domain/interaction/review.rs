use common::result::Result;

use crate::domain::interaction::{Base, Comment, Stars};
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

#[derive(Debug, Clone)]
pub struct Review {
    base: Base,
    stars: Stars,
    comment: Comment,
}

impl Review {
    pub fn new(
        reader_id: ReaderId,
        publication_id: PublicationId,
        stars: Stars,
        comment: Comment,
    ) -> Result<Self> {
        Ok(Review {
            base: Base::new(reader_id, publication_id)?,
            stars,
            comment,
        })
    }

    pub fn base(&self) -> &Base {
        &self.base
    }

    pub fn stars(&self) -> &Stars {
        &self.stars
    }

    pub fn comment(&self) -> &Comment {
        &self.comment
    }
}
