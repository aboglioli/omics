use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::interaction::ReaderPublicationId;

#[derive(Debug, Clone)]
pub struct Reading {
    base: AggregateRoot<ReaderPublicationId>,
}

impl Reading {
    pub fn new(id: ReaderPublicationId) -> Result<Self> {
        Ok(Reading {
            base: AggregateRoot::new(id),
        })
    }

    pub fn build(base: AggregateRoot<ReaderPublicationId>) -> Self {
        Reading { base }
    }

    pub fn base(&self) -> &AggregateRoot<ReaderPublicationId> {
        &self.base
    }
}
