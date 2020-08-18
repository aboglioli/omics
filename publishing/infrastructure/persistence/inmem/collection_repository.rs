use async_trait::async_trait;
use uuid::Uuid;

use common::cache::{inmem::InMemCache, Cache};
use common::error::Error;
use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::collection::{Collection, CollectionId, CollectionRepository};
use crate::mocks;

pub struct InMemCollectionRepository {
    cache: InMemCache<CollectionId, Collection>,
}

impl InMemCollectionRepository {
    pub fn new() -> Self {
        InMemCollectionRepository {
            cache: InMemCache::new(),
        }
    }

    pub async fn populated() -> Self {
        let repo = Self::new();

        repo.save(&mut mocks::empty_collection1()).await.unwrap();

        repo
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

    async fn find_all(&self) -> Result<Vec<Collection>> {
        Ok(self.cache.all().await)
    }

    async fn find_by_id(&self, id: &CollectionId) -> Result<Collection> {
        self.cache
            .get(id)
            .await
            .ok_or(Error::new("collection", "not_found"))
    }

    async fn find_by_author_id(&self, author_id: &AuthorId) -> Result<Vec<Collection>> {
        Ok(self
            .cache
            .filter(|&(_, collection)| collection.author_id() == author_id)
            .await)
    }

    async fn find_by_category_id(&self, category_id: &CategoryId) -> Result<Vec<Collection>> {
        Ok(self
            .cache
            .filter(|&(_, collection)| collection.header().category_id() == category_id)
            .await)
    }

    async fn search(&self, text: &str) -> Result<Vec<Collection>> {
        Ok(self
            .cache
            .filter(|&(_, collection)| collection.header().name().value().contains(text))
            .await)
    }

    async fn save(&self, collection: &mut Collection) -> Result<()> {
        self.cache
            .set(collection.base().id(), collection.clone())
            .await
    }
}
