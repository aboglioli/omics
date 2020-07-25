use common::error::Error;
use common::model::AggregateRoot;

use crate::domain::interaction::Like;
use crate::domain::publication::PublicationId;

pub type ReaderId = String;

pub struct Reader {
    base: AggregateRoot<ReaderId>,
    name: String,
}

impl Reader {
    pub fn new(id: ReaderId, name: &str) -> Result<Reader, Error> {
        Ok(Reader {
            base: AggregateRoot::new(id),
            name: name.to_owned(),
        })
    }

    pub fn like(&self, publication_id: PublicationId) -> Result<Like, Error> {
        Ok(Like::new(self.base.id(), publication_id)?)
    }
}
