use common::error::Error;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::category::{CategoryId, CategoryRepository};
use crate::domain::user::{UserId, UserRepository};

pub struct Delete<'a> {
    category_repo: &'a dyn CategoryRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Delete<'a> {
    pub fn new(
        category_repo: &'a dyn CategoryRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Delete {
            category_repo,
            user_repo,
        }
    }

    // TODO: events
    pub async fn exec(&self, auth_id: String, category_id: String) -> Result<CommandResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

        let mut category = self
            .category_repo
            .find_by_id(&CategoryId::new(category_id)?)
            .await?;
        category.delete()?;

        self.category_repo.save(&mut category).await?;

        Ok(CommandResponse::default())
    }
}
