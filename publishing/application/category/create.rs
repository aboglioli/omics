use serde::{Deserialize, Serialize};

use common::result::Result;

use crate::domain::admin::{AdminId, AdminRepository};
use crate::domain::category::{Category, CategoryId, CategoryRepository, Name};

#[derive(Serialize)]
pub struct CreateResponse {
    id: String,
}

#[derive(Deserialize)]
pub struct CreateCommand {
    id: String,
    name: String,
}

pub struct Create<'a> {
    admin_repo: &'a dyn AdminRepository,
    category_repo: &'a dyn CategoryRepository,
}

impl<'a> Create<'a> {
    pub fn new(
        admin_repo: &'a dyn AdminRepository,
        category_repo: &'a dyn CategoryRepository,
    ) -> Self {
        Create {
            admin_repo,
            category_repo,
        }
    }

    // TODO: events
    pub async fn exec(&self, auth_id: String, cmd: CreateCommand) -> Result<CreateResponse> {
        self.admin_repo.find_by_id(&AdminId::new(auth_id)?).await?;

        let mut category = Category::new(CategoryId::new(cmd.id)?, Name::new(cmd.name)?)?;

        self.category_repo.save(&mut category).await?;

        Ok(CreateResponse {
            id: category.base().id().to_string(),
        })
    }
}
