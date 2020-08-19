use async_trait::async_trait;
use uuid::Uuid;

use common::cache::{inmem::InMemCache, Cache};
use common::error::Error;
use common::result::Result;

use crate::domain::reader::{Reader, ReaderId, ReaderRepository};
use crate::mocks;

pub struct InMemReaderRepository {
    cache: InMemCache<ReaderId, Reader>,
}

impl InMemReaderRepository {
    pub fn new() -> Self {
        InMemReaderRepository {
            cache: InMemCache::new(),
        }
    }

    pub async fn reader() -> Self {
        let repo = Self::new();

        repo.save(&mut mocks::reader1()).await.unwrap();
        repo.save(&mut mocks::author_as_reader1()).await.unwrap();

        repo
    }
}

impl Default for InMemReaderRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ReaderRepository for InMemReaderRepository {
    async fn next_id(&self) -> Result<ReaderId> {
        let id = Uuid::new_v4();
        ReaderId::new(id.to_string())
    }

    async fn find_by_id(&self, id: &ReaderId) -> Result<Reader> {
        self.cache
            .get(id)
            .await
            .ok_or(Error::new("reader", "not_found"))
    }

    async fn save(&self, reader: &mut Reader) -> Result<()> {
        self.cache
            .set(reader.base().id().clone(), reader.clone())
            .await
    }
}
