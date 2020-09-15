use serde::{Deserialize, Serialize};

use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;
use shared::domain::user::{UserId, UserRepository};

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

        let mut category = Category::new(CategoryId::new(cmd.id)?, Name::new(cmd.name)?)?;

        self.category_repo.save(&mut category).await?;

        self.event_pub
            .publish_all(category.events().to_vec()?)
            .await?;

        Ok(CreateResponse {
            id: category.base().id().to_string(),
        })
    }
}
