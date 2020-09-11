mod repository;
pub use repository::*;

use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type AdminId = StringId;

#[derive(Debug, Clone)]
pub struct Admin {
    base: AggregateRoot<AdminId>,
}

impl Admin {
    pub fn new(id: AdminId) -> Result<Self> {
        Ok(Admin {
            base: AggregateRoot::new(id),
        })
    }

    pub fn base(&self) -> &AggregateRoot<AdminId> {
        &self.base
    }
}
