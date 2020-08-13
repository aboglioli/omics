use async_trait::async_trait;

use common::result::Result;

use crate::domain::publication::PublicationId;
use crate::domain::statistics::Statistics;

#[async_trait]
pub trait StatisticsRepository {
    async fn find_by_publication_id(&self, publication_id: &PublicationId) -> Result<Statistics>;

    async fn save(&self, statistics: &mut Statistics) -> Result<()>;
}
