use async_trait::async_trait;
use uuid::Uuid;

use common::cache::{inmem::InMemCache, Cache};
use common::error::Error;
use common::result::Result;

use crate::domain::category::{Category, CategoryId, CategoryRepository};
use crate::mocks;

pub struct InMemCategoryRepository {
    cache: InMemCache<CategoryId, Category>,
}

impl InMemCategoryRepository {
    pub fn new() -> Self {
        InMemCategoryRepository {
            cache: InMemCache::new(),
        }
    }

    pub async fn populated() -> Self {
        let repo = Self::new();

        repo.save(&mut mocks::category1()).await.unwrap();
        repo.save(&mut mocks::category2()).await.unwrap();

        repo
    }
}

impl Default for InMemCategoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CategoryRepository for InMemCategoryRepository {
    async fn next_id(&self) -> Result<CategoryId> {
        let id = Uuid::new_v4();
        CategoryId::new(id.to_string())
    }

    async fn find_by_id(&self, id: &CategoryId) -> Result<Category> {
        self.cache
            .get(id)
            .await
            .ok_or(Error::new("category", "not_found"))
    }

    async fn find_all_categories(&self) -> Result<Vec<Category>> {
        Ok(self.cache.filter(|_| true).await)
    }

    async fn save(&self, category: &mut Category) -> Result<()> {
        self.cache
            .set(category.base().id().clone(), category.clone())
            .await
    }
}
