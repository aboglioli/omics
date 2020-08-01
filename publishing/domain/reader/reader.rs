use common::error::Error;

use common::model::AggregateRoot;

use crate::domain::interaction::{Like, Read, Review, Stars};
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderEvent;

pub type ReaderId = String;

pub struct Reader {
    base: AggregateRoot<ReaderId, ReaderEvent>,
    name: String,
}

impl Reader {
    pub fn new(id: ReaderId, name: &str) -> Result<Reader, Error> {
        Ok(Reader {
            base: AggregateRoot::new(id),
            name: name.to_owned(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn read(&self, publication_id: PublicationId) -> Result<Read, Error> {
        Ok(Read::new(self.base.id(), publication_id)?)
    }

    pub fn like(&self, publication_id: PublicationId) -> Result<Like, Error> {
        Ok(Like::new(self.base.id(), publication_id)?)
    }

    pub fn review(&self, publication_id: PublicationId, stars: Stars) -> Result<Review, Error> {
        Ok(Review::new(self.base.id(), publication_id, stars)?)
    }
}
