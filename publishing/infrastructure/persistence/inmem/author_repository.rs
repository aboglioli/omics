use async_trait::async_trait;

use common::cache::Cache;
use common::error::Error;
use common::infrastructure::cache::InMemCache;
use common::result::Result;

use crate::domain::author::{Author, AuthorId, AuthorRepository};

pub struct InMemAuthorRepository {
    cache: InMemCache<AuthorId, Author>,
}

impl InMemAuthorRepository {
    pub fn new() -> Self {
        InMemAuthorRepository {
            cache: InMemCache::new(),
        }
    }
}

impl Default for InMemAuthorRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AuthorRepository for InMemAuthorRepository {
    async fn find_all(&self) -> Result<Vec<Author>> {
        Ok(self.cache.all().await)
    }

    async fn find_by_id(&self, id: &AuthorId) -> Result<Author> {
        self.cache
            .get(id)
            .await
            .ok_or_else(|| Error::not_found("author"))
    }

    async fn save(&self, author: &mut Author) -> Result<()> {
        if author.base().deleted_at().is_none() {
            self.cache
                .set(author.base().id().clone(), author.clone())
                .await
        } else {
            self.cache.delete(author.base().id()).await
        }
    }

    async fn delete(&self, id: &AuthorId) -> Result<()> {
        self.cache.delete(id).await
    }
}
