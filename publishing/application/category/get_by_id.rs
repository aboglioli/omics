use common::result::Result;

use crate::application::dtos::CategoryDto;
use crate::domain::category::{CategoryId, CategoryRepository};

pub struct GetById<'a, CRepo> {
    category_repo: &'a CRepo,
}

impl<'a, CRepo> GetById<'a, CRepo>
where
    CRepo: CategoryRepository,
{
    pub fn new(category_repo: &'a CRepo) -> Self {
        GetById { category_repo }
    }

    pub async fn exec(&self, category_id: String) -> Result<CategoryDto> {
        let category_id = CategoryId::new(category_id)?;
        let category = self.category_repo.find_by_id(&category_id).await?;

        Ok(CategoryDto::new(&category))
    }
}
