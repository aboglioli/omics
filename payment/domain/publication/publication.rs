use common::error::Error;
use common::model::AggregateRoot;

use crate::domain::user::{User, UserID};

pub type PublicationID = String;

pub struct Publication {
    base: AggregateRoot<PublicationID>,
    author_id: UserID,
    name: String,
}

impl Publication {
    pub fn new(id: PublicationID, author_id: UserID, name: &str) -> Result<Publication, Error> {
        Ok(Publication {
            base: AggregateRoot::new(id),
            author_id,
            name: name.to_owned(),
        })
    }
}
