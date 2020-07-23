use common::error::Error;
use common::model::{Entity, ID};

use crate::domain::user::{User, UserID};

pub type PublicationID = String;

pub struct Publication {
    id: ID<PublicationID>,
    author_id: UserID,
    name: String,
}

impl Publication {
    pub fn new(id: PublicationID, author_id: UserID, name: &str) -> Result<Publication, Error> {
        Ok(Publication {
            id: ID::new(id),
            author_id,
            name: name.to_owned(),
        })
    }
}

impl Entity<PublicationID> for Publication {
    fn id(&self) -> &ID<PublicationID> {
        &self.id
    }
}
