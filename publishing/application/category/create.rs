use serde::{Deserialize, Serialize};

use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;
use identity::domain::user::UserRepository;
use identity::UserIdAndRole;

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
}

impl<'a> Create<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        category_repo: &'a dyn CategoryRepository,
    ) -> Self {
        Create {
            event_pub,
            category_repo,
        }
    }

    pub async fn exec(
        &self,
        (_auth_id, auth_role): UserIdAndRole,
        cmd: CreateCommand,
    ) -> Result<CreateResponse> {
        if !auth_role.can("create_category") {
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
