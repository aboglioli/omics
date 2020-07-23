use common::error::Error;
use common::model::{Entity, ID};

use crate::domain::interaction::Like;
use crate::domain::publication::PublicationID;

pub type ReaderID = String;

pub struct Reader {
    id: ID<ReaderID>,
    name: String,
}

impl Reader {
    pub fn new(id: ReaderID, name: &str) -> Result<Reader, Error> {
        Ok(Reader {
            id: ID::new(id),
            name: name.to_owned(),
        })
    }

    pub fn like(&self, publication_id: PublicationID) -> Result<Like, Error> {
        Ok(Like::new(self.id().value(), publication_id)?)
    }
}

impl Entity<ReaderID> for Reader {
    fn id(&self) -> &ID<ReaderID> {
        &self.id
    }
}
