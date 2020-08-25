use async_trait::async_trait;

use crate::cache::Cache;

use crate::event::{Event, EventId, EventRepository};
use crate::infrastructure::cache::InMemCache;
use crate::result::Result;

pub struct InMemEventRepository {
    cache: InMemCache<EventId, Event>,
}

impl InMemEventRepository {
    pub fn new() -> Self {
        InMemEventRepository {
            cache: InMemCache::new(),
        }
    }
}

impl Default for InMemEventRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventRepository for InMemEventRepository {
    async fn find_all(&self) -> Result<Vec<Event>> {
        Ok(self.cache.all().await)
    }

    async fn find_from(&self, _id: &EventId) -> Result<Vec<Event>> {
        Ok(self.cache.all().await)
    }

    async fn save(&self, event: &Event) -> Result<()> {
        self.cache.set(event.id().clone(), event.clone()).await
    }
}
