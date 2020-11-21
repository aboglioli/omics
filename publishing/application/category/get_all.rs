use serde::Serialize;

use common::error::Error;
use common::result::Result;
use identity::UserIdAndRole;

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

    pub async fn exec(&self, user_id_and_role: Option<UserIdAndRole>) -> Result<GetAllResponse> {
        if let Some((_auth_id, auth_role)) = user_id_and_role {
            if !auth_role.can("get_all_categories") {
                return Err(Error::unauthorized());
            }
        }

        let categories = self.category_repo.find_all().await?;
        Ok(GetAllResponse {
            categories: categories.iter().map(CategoryDto::from).collect(),
        })
    }
}
