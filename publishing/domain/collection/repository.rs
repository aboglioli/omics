use async_trait::async_trait;

use common::result::Result;

use crate::domain::collection::{Collection, CollectionId};

#[async_trait]
pub trait CollectionRepository {
    async fn next_id(&self) -> Result<CollectionId>;

    async fn find_by_id(&self, id: &CollectionId) -> Result<Collection>;

    async fn save(&self, collection: &mut Collection) -> Result<()>;
}
