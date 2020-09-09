use common::request::CommandResponse;
use common::result::Result;

use crate::domain::admin::{AdminId, AdminRepository};
use crate::domain::category::{CategoryId, CategoryRepository};

pub struct Delete<'a> {
    admin_repo: &'a dyn AdminRepository,
    category_repo: &'a dyn CategoryRepository,
}

impl<'a> Delete<'a> {
    pub fn new(
        admin_repo: &'a dyn AdminRepository,
        category_repo: &'a dyn CategoryRepository,
    ) -> Self {
        Delete {
            admin_repo,
            category_repo,
        }
    }

    // TODO: events
    pub async fn exec(&self, auth_id: String, category_id: String) -> Result<CommandResponse> {
        self.admin_repo.find_by_id(&AdminId::new(auth_id)?).await?;

        let mut category = self
            .category_repo
            .find_by_id(&CategoryId::new(category_id)?)
            .await?;
        category.delete()?;

        self.category_repo.save(&mut category).await?;

        Ok(CommandResponse::default())
    }
}
