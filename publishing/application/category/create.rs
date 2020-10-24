use serde::{Deserialize, Serialize};

use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};

use crate::domain::category::{Category, CategoryRepository, Name};

#[derive(Serialize)]
pub struct CreateResponse {
    id: String,
}

#[derive(Deserialize)]
pub struct CreateCommand {
    name: String,
}

pub struct Create<'a> {
    event_pub: &'a dyn EventPublisher,

    category_repo: &'a dyn CategoryRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Create<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        category_repo: &'a dyn CategoryRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Create {
            event_pub,
            category_repo,
            user_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, cmd: CreateCommand) -> Result<CreateResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

        let mut category = Category::new(Name::new(cmd.name)?)?;

        if self
            .category_repo
            .find_by_id(category.base().id())
            .await
            .is_ok()
        {
            return Err(Error::new("category", "already_exists"));
        }

        self.category_repo.save(&mut category).await?;

        self.event_pub
            .publish_all(category.events().to_vec()?)
            .await?;

        Ok(CreateResponse {
            id: category.base().id().to_string(),
        })
    }
}
