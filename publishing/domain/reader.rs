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
    subscribed: bool,
    preferences: Preferences,
}

impl Reader {
    pub fn new(id: ReaderId) -> Result<Self> {
        Ok(Reader {
            base: AggregateRoot::new(id),
            subscribed: false,
            preferences: Preferences::default(),
        })
    }

    pub fn base(&self) -> &AggregateRoot<ReaderId, Event> {
        &self.base
    }

    pub fn is_subscribed(&self) -> bool {
        self.subscribed
    }

    pub fn preferences(&self) -> &Preferences {
        &self.preferences
    }

    pub fn preferences_mut(&mut self) -> &mut Preferences {
        // TODO: improve this
        self.base.update();
        &mut self.preferences
    }

    pub fn subscribe(&mut self) -> Result<()> {
        self.subscribed = true;
        self.base.update();
        Ok(())
    }

    pub fn unsubscribe(&mut self) -> Result<()> {
        self.subscribed = false;
        self.base.update();
        Ok(())
    }

    pub fn set_preferences(&mut self, preferences: Preferences) -> Result<()> {
        self.preferences = preferences;
        self.base.update();
        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.base.delete();
        Ok(())
    }
}
