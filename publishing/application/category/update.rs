use serde::Deserialize;

use common::request::CommandResponse;
use common::result::Result;

use crate::domain::admin::{AdminId, AdminRepository};
use crate::domain::category::{CategoryId, CategoryRepository, Name};

#[derive(Deserialize)]
pub struct UpdateCommand {
    name: String,
}

pub struct Update<'a> {
    admin_repo: &'a dyn AdminRepository,
    category_repo: &'a dyn CategoryRepository,
}

impl<'a> Update<'a> {
    pub fn new(
        admin_repo: &'a dyn AdminRepository,
        category_repo: &'a dyn CategoryRepository,
    ) -> Self {
        Update {
            admin_repo,
            category_repo,
        }
    }

    // TODO: events
    pub async fn exec(
        &self,
        auth_id: String,
        category_id: String,
        cmd: UpdateCommand,
    ) -> Result<CommandResponse> {
        self.admin_repo.find_by_id(&AdminId::new(auth_id)?).await?;

        let mut category = self
            .category_repo
            .find_by_id(&CategoryId::new(category_id)?)
            .await?;
        category.set_name(Name::new(cmd.name)?)?;

        self.category_repo.save(&mut category).await?;

        Ok(CommandResponse::default())
    }
}
