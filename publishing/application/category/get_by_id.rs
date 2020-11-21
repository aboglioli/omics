use common::error::Error;
use common::result::Result;
use identity::UserIdAndRole;

use crate::application::dtos::CategoryDto;
use crate::domain::category::{CategoryId, CategoryRepository};

pub struct GetById<'a> {
    category_repo: &'a dyn CategoryRepository,
}

impl<'a> GetById<'a> {
    pub fn new(category_repo: &'a dyn CategoryRepository) -> Self {
        GetById { category_repo }
    }

    pub async fn exec(
        &self,
        user_id_and_role: Option<UserIdAndRole>,
        category_id: String,
    ) -> Result<CategoryDto> {
        if let Some((_auth_id, auth_role)) = user_id_and_role {
            if !auth_role.can("get_category") {
                return Err(Error::unauthorized());
            }
        }

        let category_id = CategoryId::new(category_id)?;
        let category = self.category_repo.find_by_id(&category_id).await?;

        Ok(CategoryDto::from(&category))
    }
}
