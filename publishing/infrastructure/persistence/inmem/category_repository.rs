use async_trait::async_trait;

use common::cache::Cache;
use common::error::Error;
use common::infrastructure::cache::InMemCache;
use common::result::Result;

use crate::domain::category::{Category, CategoryId, CategoryRepository};

pub struct InMemCategoryRepository {
    cache: InMemCache<CategoryId, Category>,
}

impl InMemCategoryRepository {
    pub fn new() -> Self {
        InMemCategoryRepository {
            cache: InMemCache::new(),
        }
    }
}

impl Default for InMemCategoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CategoryRepository for InMemCategoryRepository {
    async fn find_all(&self) -> Result<Vec<Category>> {
        Ok(self.cache.filter(|_| true).await)
    }

    async fn find_by_id(&self, id: &CategoryId) -> Result<Category> {
        self.cache
            .get(id)
            .await
            .ok_or_else(|| Error::not_found("category"))
    }

    async fn save(&self, category: &mut Category) -> Result<()> {
        if category.base().deleted_at().is_none() {
            self.cache
                .set(category.base().id().clone(), category.clone())
                .await
        } else {
            self.cache.delete(category.base().id()).await
        }
    }
}
