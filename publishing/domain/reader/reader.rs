use common::error::Error;
use common::model::AggregateRoot;

use crate::domain::interaction::Like;
use crate::domain::publication::PublicationID;

pub type ReaderID = String;

pub struct Reader {
    base: AggregateRoot<ReaderID>,
    name: String,
}

impl Reader {
    pub fn new(id: ReaderID, name: &str) -> Result<Reader, Error> {
        Ok(Reader {
            base: AggregateRoot::new(id),
            name: name.to_owned(),
        })
    }

    pub fn like(&self, publication_id: PublicationID) -> Result<Like, Error> {
        Ok(Like::new(self.base.id(), publication_id)?)
    }
}
