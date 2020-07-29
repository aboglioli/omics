use async_trait::async_trait;

use common::error::Error;

use crate::domain::category::{Category, CategoryId};

#[async_trait]
pub trait CategoryRepository {
    async fn find_all_categories(&self) -> Result<Vec<Category>, Error>;
    async fn find_category_by_id(&self, id: &CategoryId) -> Result<Category, Error>;
}
