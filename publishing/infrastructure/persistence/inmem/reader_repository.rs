use async_trait::async_trait;
use uuid::Uuid;

use common::cache::{inmem::InMemCache, Cache};
use common::error::Error;
use common::result::Result;

use crate::domain::reader::{Reader, ReaderId, ReaderRepository};

pub struct InMemReaderRepository {
    cache: InMemCache<ReaderId, Reader>,
}

impl InMemReaderRepository {
    pub fn new() -> Self {
        InMemReaderRepository {
            cache: InMemCache::new(),
        }
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
        self.cache.set(reader.base().id(), reader.clone()).await
    }
}
