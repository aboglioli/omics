use serde::Deserialize;

use common::error::Error;
use common::request::CommandResponse;
use common::result::Result;
use shared::domain::user::{UserId, UserRepository};

use crate::domain::category::{CategoryId, CategoryRepository, Name};

#[derive(Deserialize)]
pub struct UpdateCommand {
    name: String,
}

pub struct Update<'a> {
    category_repo: &'a dyn CategoryRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Update<'a> {
    pub fn new(
        category_repo: &'a dyn CategoryRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Update {
            category_repo,
            user_repo,
        }
    }

    // TODO: events
    pub async fn exec(
        &self,
        auth_id: String,
        category_id: String,
        cmd: UpdateCommand,
    ) -> Result<CommandResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

        let mut category = self
            .category_repo
            .find_by_id(&CategoryId::new(category_id)?)
            .await?;
        category.set_name(Name::new(cmd.name)?)?;

        self.category_repo.save(&mut category).await?;

        Ok(CommandResponse::default())
    }
}
