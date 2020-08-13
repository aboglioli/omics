use async_trait::async_trait;
use uuid::Uuid;

use common::cache::{inmem::InMemCache, Cache};
use common::error::Error;
use common::result::Result;

use crate::domain::collection::{Collection, CollectionId, CollectionRepository};

pub struct InMemCollectionRepository {
    cache: InMemCache<CollectionId, Collection>,
}

impl InMemCollectionRepository {
    pub fn new() -> Self {
        InMemCollectionRepository {
            cache: InMemCache::new(),
        }
    }
}

impl Default for InMemCollectionRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CollectionRepository for InMemCollectionRepository {
    async fn next_id(&self) -> Result<CollectionId> {
        let id = Uuid::new_v4();
        CollectionId::new(id.to_string())
    }

    async fn find_by_id(&self, id: &CollectionId) -> Result<Collection> {
        self.cache
            .get(id)
            .await
            .ok_or(Error::new("collection", "not_found"))
    }

    async fn save(&self, collection: &mut Collection) -> Result<()> {
        self.cache
            .set(collection.base().id(), collection.clone())
            .await
    }
}
