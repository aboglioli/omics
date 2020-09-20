use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::interaction::ReaderPublicationId;

#[derive(Debug, Clone)]
pub struct View {
    base: AggregateRoot<ReaderPublicationId>,
    unique: bool,
}

impl View {
    pub fn new(id: ReaderPublicationId, unique: bool) -> Result<Self> {
        Ok(View {
            base: AggregateRoot::new(id),
            unique,
        })
    }

    pub fn build(base: AggregateRoot<ReaderPublicationId>, unique: bool) -> Self {
        View { base, unique }
    }

    pub fn base(&self) -> &AggregateRoot<ReaderPublicationId> {
        &self.base
    }

    pub fn is_unique(&self) -> bool {
        self.unique
    }
}
