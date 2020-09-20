use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::interaction::ReaderPublicationId;

#[derive(Debug, Clone)]
pub struct Like {
    base: AggregateRoot<ReaderPublicationId>,
}

impl Like {
    pub fn new(id: ReaderPublicationId) -> Result<Self> {
        Ok(Like {
            base: AggregateRoot::new(id),
        })
    }

    pub fn build(base: AggregateRoot<ReaderPublicationId>) -> Self {
        Like { base }
    }

    pub fn base(&self) -> &AggregateRoot<ReaderPublicationId> {
        &self.base
    }
}
