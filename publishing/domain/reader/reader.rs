use common::error::Error;
use common::event::Event;
use common::result::Result;

use common::model::AggregateRoot;

use crate::domain::interaction::{Like, Reading, Review, Stars, View};
use crate::domain::publication::Publication;

pub type ReaderId = String;

pub struct Reader {
    base: AggregateRoot<ReaderId, Event>,
    name: String,
    subscribed: bool,
}

impl Reader {
    pub fn new(id: ReaderId, name: &str) -> Result<Reader> {
        Ok(Reader {
            base: AggregateRoot::new(id),
            name: name.to_owned(),
            subscribed: false,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_subscribed(&self) -> bool {
        self.subscribed
    }

    pub fn subscribe(&mut self) -> Result<()> {
        self.subscribed = true;
        Ok(())
    }

    pub fn view(&self, publication: &Publication) -> Result<View> {
        Ok(View::new(self.base.id(), publication.base().id())?)
    }

    pub fn read(&self, publication: &Publication) -> Result<Reading> {
        if publication.has_contract() && !self.subscribed {
            return Err(Error::new("reader", "not_subscribed"));
        }

        Ok(Reading::new(self.base.id(), publication.base().id())?)
    }

    pub fn like(&self, publication: &Publication) -> Result<Like> {
        if publication.has_contract() && !self.subscribed {
            return Err(Error::new("reader", "not_subscribed"));
        }

        Ok(Like::new(self.base.id(), publication.base().id())?)
    }

    pub fn review(&self, publication: &Publication, stars: Stars) -> Result<Review> {
        if publication.has_contract() && !self.subscribed {
            return Err(Error::new("reader", "not_subscribed"));
        }

        Ok(Review::new(self.base.id(), publication.base().id(), stars)?)
    }
}
