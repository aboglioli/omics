use async_trait::async_trait;

use common::cache::Cache;
use common::error::Error;
use common::infrastructure::cache::InMemCache;
use common::result::Result;

use crate::domain::admin::{Admin, AdminId, AdminRepository};

pub struct InMemAdminRepository {
    cache: InMemCache<AdminId, Admin>,
}

impl InMemAdminRepository {
    pub fn new() -> Self {
        InMemAdminRepository {
            cache: InMemCache::new(),
        }
    }
}

impl Default for InMemAdminRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AdminRepository for InMemAdminRepository {
    async fn find_by_id(&self, id: &AdminId) -> Result<Admin> {
        self.cache
            .get(id)
            .await
            .ok_or_else(|| Error::not_found("admin"))
    }

    async fn save(&self, admin: &mut Admin) -> Result<()> {
        self.cache
            .set(admin.base().id().clone(), admin.clone())
            .await
    }
}
