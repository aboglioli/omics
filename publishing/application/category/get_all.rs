use serde::Serialize;

use common::result::Result;

use crate::application::dtos::CategoryDto;
use crate::domain::category::CategoryRepository;

#[derive(Serialize)]
pub struct GetAllResponse {
    pub categories: Vec<CategoryDto>,
}

pub struct GetAll<'a> {
    category_repo: &'a dyn CategoryRepository,
}

impl<'a> GetAll<'a> {
    pub fn new(category_repo: &'a dyn CategoryRepository) -> Self {
        GetAll { category_repo }
    }

    pub async fn exec(&self) -> Result<GetAllResponse> {
        let categories = self.category_repo.find_all_categories().await?;
        Ok(GetAllResponse {
            categories: categories.iter().map(CategoryDto::from).collect(),
        })
    }
}
