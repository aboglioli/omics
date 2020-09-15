use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::interaction::ReaderCollectionId;

#[derive(Debug, Clone)]
pub struct CollectionFavorite {
    base: AggregateRoot<ReaderCollectionId>,
}

impl CollectionFavorite {
    pub fn new(id: ReaderCollectionId) -> Result<Self> {
        Ok(CollectionFavorite {
            base: AggregateRoot::new(id),
        })
    }

    pub fn base(&self) -> &AggregateRoot<ReaderCollectionId> {
        &self.base
    }
}
