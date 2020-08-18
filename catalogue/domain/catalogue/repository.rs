use async_trait::async_trait;

use common::result::Result;

use crate::domain::catalogue::Catalogue;

#[async_trait]
pub trait CatalogueRepository: Sync + Send {
    async fn find(&self) -> Result<Catalogue>;
    async fn save(&self, catalogue: &mut Catalogue) -> Result<()>;
}
