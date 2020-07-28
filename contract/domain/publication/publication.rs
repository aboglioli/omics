use common::domain::user::UserId;
use common::error::Error;
use common::model::{AggregateRoot, DefaultEvent};

pub type PublicationId = String;

pub struct Publication {
    base: AggregateRoot<PublicationId, DefaultEvent>,
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
