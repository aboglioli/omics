use common::error::Error;
use common::model::AggregateRoot;

use crate::domain::user::{User, UserId};

pub type PublicationId = String;

pub struct Publication {
    base: AggregateRoot<PublicationId>,
    author_id: UserId,
    name: String,
}

impl Publication {
    pub fn new(id: PublicationId, author_id: UserId, name: &str) -> Result<Publication, Error> {
        Ok(Publication {
            base: AggregateRoot::new(id),
            author_id,
            name: name.to_owned(),
        })
    }
}
