use identity::domain::user::UserId;

use common::event::Event;
use common::model::AggregateRoot;
use common::result::Result;

pub type PublicationId = String;

pub struct Publication {
    base: AggregateRoot<PublicationId, Event>,
    author_id: UserId,
    name: String,
}

impl Publication {
    pub fn new(id: PublicationId, author_id: UserId, name: &str) -> Result<Publication> {
        Ok(Publication {
            base: AggregateRoot::new(id),
            author_id,
            name: name.to_owned(),
        })
    }

    pub fn base(&self) -> &AggregateRoot<PublicationId, Event> {
        &self.base
    }

    pub fn author_id(&self) -> &UserId {
        &self.author_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
