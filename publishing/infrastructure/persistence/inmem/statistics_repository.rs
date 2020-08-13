use async_trait::async_trait;

use common::cache::{inmem::InMemCache, Cache};
use common::error::Error;
use common::result::Result;

use crate::domain::publication::PublicationId;
use crate::domain::statistics::{Statistics, StatisticsRepository};

pub struct InMemStatisticsRepository {
    cache: InMemCache<PublicationId, Statistics>,
}

impl InMemStatisticsRepository {
    pub fn new() -> Self {
        InMemStatisticsRepository {
            cache: InMemCache::new(),
        }
    }
}

impl Default for InMemStatisticsRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StatisticsRepository for InMemStatisticsRepository {
    async fn find_by_publication_id(&self, publication_id: &PublicationId) -> Result<Statistics> {
        self.cache
            .get(publication_id)
            .await
            .ok_or(Error::new("statistics", "not_found"))
    }

    async fn save(&self, statistics: &mut Statistics) -> Result<()> {
        self.cache
            .set(statistics.publication_id().clone(), statistics.clone())
            .await
    }
}
