use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use identity::UserIdAndRole;

use crate::domain::category::{CategoryId, CategoryRepository, Name};

#[derive(Deserialize)]
pub struct UpdateCommand {
    name: String,
}

pub struct Update<'a> {
    event_pub: &'a dyn EventPublisher,
    category_repo: &'a dyn CategoryRepository,
}

impl<'a> Update<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        category_repo: &'a dyn CategoryRepository,
    ) -> Self {
        Update {
            event_pub,
            category_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        category_id: String,
        cmd: UpdateCommand,
    ) -> Result<CommandResponse> {
        if !auth_role.can("update_category") {
            return Err(Error::unauthorized());
        }

        let mut category = self
            .category_repo
            .find_by_id(&CategoryId::new(category_id)?)
            .await?;
        category.set_name(Name::new(cmd.name)?)?;

        self.category_repo.save(&mut category).await?;

        self.event_pub
            .publish_all(category.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
