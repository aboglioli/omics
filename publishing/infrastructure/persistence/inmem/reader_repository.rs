use async_trait::async_trait;

use common::cache::Cache;
use common::error::Error;
use common::infrastructure::cache::InMemCache;
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
    async fn find_by_id(&self, id: &ReaderId) -> Result<Reader> {
        self.cache
            .get(id)
            .await
            .ok_or_else(|| Error::not_found("reader"))
    }

    async fn save(&self, reader: &mut Reader) -> Result<()> {
        self.cache
            .set(reader.base().id().clone(), reader.clone())
            .await
    }
}
