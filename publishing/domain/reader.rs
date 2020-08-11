mod reader_repository;
pub use reader_repository::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type ReaderId = StringId;

pub struct Reader {
    base: AggregateRoot<ReaderId, Event>,
    name: String,
    subscribed: bool,
}

impl Reader {
    pub fn new(id: ReaderId, name: &str) -> Result<Self> {
        Ok(Reader {
            base: AggregateRoot::new(id),
            name: name.to_owned(),
            subscribed: false,
        })
    }

    pub fn base(&self) -> &AggregateRoot<ReaderId, Event> {
        &self.base
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

    pub fn unsubscribe(&mut self) -> Result<()> {
        self.subscribed = false;
        Ok(())
    }
}
