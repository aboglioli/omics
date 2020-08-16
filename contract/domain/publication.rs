use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type PublicationId = StringId;

#[derive(Debug, Clone)]
pub struct Publication {
    base: AggregateRoot<PublicationId, Event>,
}

impl Publication {
    pub fn new(id: PublicationId) -> Result<Self> {
        Ok(Publication {
            base: AggregateRoot::new(id),
        })
    }

    pub fn base(&self) -> &AggregateRoot<PublicationId, Event> {
        &self.base
    }
}
