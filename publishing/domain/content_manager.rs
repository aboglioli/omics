mod repository;
pub use repository::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type ContentManagerId = StringId;

#[derive(Debug, Clone)]
pub struct ContentManager {
    base: AggregateRoot<ContentManagerId, Event>,
}

impl ContentManager {
    pub fn new(id: ContentManagerId) -> Result<Self> {
        Ok(ContentManager {
            base: AggregateRoot::new(id),
        })
    }

    pub fn base(&self) -> &AggregateRoot<ContentManagerId, Event> {
        &self.base
    }
}
