use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::interaction::ReaderAuthorId;

#[derive(Debug, Clone)]
pub struct Follow {
    base: AggregateRoot<ReaderAuthorId>,
}

impl Follow {
    pub fn new(id: ReaderAuthorId) -> Result<Self> {
        Ok(Follow {
            base: AggregateRoot::new(id),
        })
    }

    pub fn base(&self) -> &AggregateRoot<ReaderAuthorId> {
        &self.base
    }
}
