use async_trait::async_trait;

use common::result::Result;

use crate::domain::category::{Category, CategoryId};

#[async_trait]
pub trait CategoryRepository {
    async fn next_id(&self) -> Result<CategoryId>;

    async fn find_by_id(&self, id: &CategoryId) -> Result<Category>;
    async fn find_all_categories(&self) -> Result<Vec<Category>>;

    async fn save(&self, category: &mut Category) -> Result<()>;
}
