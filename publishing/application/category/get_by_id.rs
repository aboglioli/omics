use common::result::Result;

use crate::application::dtos::CategoryDto;
use crate::domain::category::{CategoryId, CategoryRepository};

pub struct GetById<'a> {
    category_repo: &'a dyn CategoryRepository,
}

impl<'a> GetById<'a> {
    pub fn new(category_repo: &'a dyn CategoryRepository) -> Self {
        GetById { category_repo }
    }

    pub async fn exec(&self, category_id: String) -> Result<CategoryDto> {
        let category_id = CategoryId::new(category_id)?;
        let category = self.category_repo.find_by_id(&category_id).await?;

        Ok(CategoryDto::new(&category))
    }
}
