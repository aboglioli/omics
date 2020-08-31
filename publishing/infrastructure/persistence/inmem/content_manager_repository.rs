use async_trait::async_trait;
use uuid::Uuid;

use common::cache::Cache;
use common::error::Error;
use common::infrastructure::cache::InMemCache;
use common::result::Result;

use crate::domain::content_manager::{ContentManager, ContentManagerId, ContentManagerRepository};

pub struct InMemContentManagerRepository {
    cache: InMemCache<ContentManagerId, ContentManager>,
}

impl InMemContentManagerRepository {
    pub fn new() -> Self {
        InMemContentManagerRepository {
            cache: InMemCache::new(),
        }
    }
}

impl Default for InMemContentManagerRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ContentManagerRepository for InMemContentManagerRepository {
    async fn next_id(&self) -> Result<ContentManagerId> {
        let id = Uuid::new_v4();
        ContentManagerId::new(id.to_string())
    }

    async fn find_by_id(&self, id: &ContentManagerId) -> Result<ContentManager> {
        self.cache
            .get(id)
            .await
            .ok_or_else(|| Error::not_found("content_manager"))
    }

    async fn save(&self, content_manager: &mut ContentManager) -> Result<()> {
        self.cache
            .set(content_manager.base().id().clone(), content_manager.clone())
            .await
    }
}
