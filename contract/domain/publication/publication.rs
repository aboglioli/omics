use common::domain::user::UserId;
use common::error::Error;
use common::event::BasicEvent;
use common::model::AggregateRoot;

pub type PublicationId = String;

pub struct Publication {
    base: AggregateRoot<PublicationId, BasicEvent>,
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
