use async_trait::async_trait;

use common::result::Result;

use crate::domain::catalogue::Collection;

#[async_trait]
pub trait CollectionService: Sync + Send {
    async fn get_by_id(&self, id: &str) -> Result<Collection>;
}
