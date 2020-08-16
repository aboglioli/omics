use async_trait::async_trait;
use uuid::Uuid;

use common::cache::{inmem::InMemCache, Cache};
use common::error::Error;
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
    async fn next_id(&self) -> Result<AuthorId> {
        let id = Uuid::new_v4();
        AuthorId::new(id.to_string())
    }

    async fn find_by_id(&self, id: &AuthorId) -> Result<Author> {
        self.cache
            .get(id)
            .await
            .ok_or(Error::new("author", "not_found"))
    }

    async fn search(&self, text: &str) -> Result<Vec<Author>> {
        Ok(self
            .cache
            .filter(|&(_, author)| {
                author.username().contains(text)
                    || author.name().contains(text)
                    || author.lastname().contains(text)
            })
            .await)
    }

    async fn save(&self, author: &mut Author) -> Result<()> {
        self.cache.set(author.base().id(), author.clone()).await
    }
}
