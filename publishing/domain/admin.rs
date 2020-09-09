mod repository;
pub use repository::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type AdminId = StringId;

#[derive(Debug, Clone)]
pub struct Admin {
    base: AggregateRoot<AdminId, Event>,
}

impl Admin {
    pub fn new(id: AdminId) -> Result<Self> {
        Ok(Admin {
            base: AggregateRoot::new(id),
        })
    }

    pub fn base(&self) -> &AggregateRoot<AdminId, Event> {
        &self.base
    }
}
