use async_trait::async_trait;
use tokio::sync::Mutex;

use common::result::Result;

use crate::domain::catalogue::{Catalogue, CatalogueId, CatalogueRepository};

pub struct InMemCatalogueRepository {
    catalogue: Mutex<Catalogue>,
}

impl InMemCatalogueRepository {
    pub fn new() -> Self {
        InMemCatalogueRepository {
            catalogue: Mutex::new(
                Catalogue::new(CatalogueId::new("#catalogue01").unwrap()).unwrap(),
            ),
        }
    }
}

#[async_trait]
impl CatalogueRepository for InMemCatalogueRepository {
    async fn find(&self) -> Result<Catalogue> {
        Ok(self.catalogue.lock().await.clone())
    }

    async fn save(&self, _catalogue: &mut Catalogue) -> Result<()> {
        let mut catalogue = self.catalogue.lock().await;
        *catalogue = catalogue.clone();
        Ok(())
    }
}
