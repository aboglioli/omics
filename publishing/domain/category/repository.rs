use async_trait::async_trait;
use uuid::Uuid;

use common::result::Result;

use crate::domain::category::{Category, CategoryId};

#[async_trait]
pub trait CategoryRepository: Sync + Send {
    async fn next_id(&self) -> Result<CategoryId> {
        CategoryId::new(Uuid::new_v4().to_string())
    }

    async fn find_all(&self) -> Result<Vec<Category>>;
    async fn find_by_id(&self, id: &CategoryId) -> Result<Category>;

    async fn save(&self, category: &mut Category) -> Result<()>;

    async fn delete(&self, id: &CategoryId) -> Result<()>;
}
