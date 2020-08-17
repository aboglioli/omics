use serde::Serialize;

use common::result::Result;

use crate::application::dtos::CategoryDto;
use crate::domain::category::CategoryRepository;

#[derive(Serialize)]
pub struct GetAllResponse {
    categories: Vec<CategoryDto>,
}

pub struct GetAll<'a, CRepo> {
    category_repo: &'a CRepo,
}

impl<'a, CRepo> GetAll<'a, CRepo>
where
    CRepo: CategoryRepository,
{
    pub fn new(category_repo: &'a CRepo) -> Self {
        GetAll { category_repo }
    }

    pub async fn exec(&self) -> Result<GetAllResponse> {
        let categories = self.category_repo.find_all_categories().await?;
        Ok(GetAllResponse {
            categories: categories
                .iter()
                .map(|category| CategoryDto::new(category))
                .collect(),
        })
    }
}
