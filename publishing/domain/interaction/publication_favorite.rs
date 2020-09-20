use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::interaction::ReaderPublicationId;

#[derive(Debug, Clone)]
pub struct PublicationFavorite {
    base: AggregateRoot<ReaderPublicationId>,
}

impl PublicationFavorite {
    pub fn new(id: ReaderPublicationId) -> Result<Self> {
        Ok(PublicationFavorite {
            base: AggregateRoot::new(id),
        })
    }

    pub fn build(base: AggregateRoot<ReaderPublicationId>) -> Self {
        PublicationFavorite { base }
    }

    pub fn base(&self) -> &AggregateRoot<ReaderPublicationId> {
        &self.base
    }
}
