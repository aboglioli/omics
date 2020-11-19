use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::category::{CategoryId, CategoryRepository};

pub struct Delete<'a> {
    event_pub: &'a dyn EventPublisher,

    category_repo: &'a dyn CategoryRepository,
}

impl<'a> Delete<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        category_repo: &'a dyn CategoryRepository,
    ) -> Self {
        Delete {
            event_pub,
            category_repo,
        }
    }

    pub async fn exec(
        &self,
        (_auth_id, auth_role): UserIdAndRole,
        category_id: String,
    ) -> Result<CommandResponse> {
        if !auth_role.can("delete_category") {
            return Err(Error::unauthorized());
        }

        let mut category = self
            .category_repo
            .find_by_id(&CategoryId::new(category_id)?)
            .await?;
        category.delete()?;

        self.category_repo.delete(category.base().id()).await?;

        self.event_pub
            .publish_all(category.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
