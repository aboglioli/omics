mod preferences;
mod repository;
pub use preferences::*;
pub use repository::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type ReaderId = StringId;

#[derive(Debug, Clone)]
pub struct Reader {
    base: AggregateRoot<ReaderId, Event>,
    username: String,
    name: String,
    lastname: String,
    subscribed: bool,
    preferences: Preferences,
}

impl Reader {
    pub fn new<S: Into<String>>(id: ReaderId, username: S, name: S, lastname: S) -> Result<Self> {
        Ok(Reader {
            base: AggregateRoot::new(id),
            username: username.into(),
            name: name.into(),
            lastname: lastname.into(),
            subscribed: false,
            preferences: Preferences::default(),
        })
    }

    pub fn base(&self) -> &AggregateRoot<ReaderId, Event> {
        &self.base
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lastname(&self) -> &str {
        &self.lastname
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
