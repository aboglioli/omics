use serde::{Deserialize, Serialize};

use common::error::Error;
use common::result::Result;

use crate::application::dtos::UserDto;
use crate::domain::role::RoleId;
use crate::domain::user::{UserId, UserRepository};

#[derive(Deserialize)]
pub struct SearchCommand {
    pub role_id: Option<String>,
}

impl SearchCommand {
    pub fn is_empty(&self) -> bool {
        self.role_id.is_none()
    }
}

#[derive(Serialize)]
pub struct SearchResponse {
    users: Vec<UserDto>,
}

pub struct Search<'a> {
    user_repo: &'a dyn UserRepository,
}

impl<'a> Search<'a> {
    pub fn new(user_repo: &'a dyn UserRepository) -> Self {
        Search { user_repo }
    }

    pub async fn exec(&self, auth_id: String, cmd: SearchCommand) -> Result<SearchResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.role().is("admin") {
            return Err(Error::unauthorized());
        }

        let mut users = Vec::new();

        if cmd.is_empty() {
            users.extend(self.user_repo.find_all().await?);
        } else {
            if let Some(role_id) = cmd.role_id {
                users.extend(
                    self.user_repo
                        .find_by_role_id(&RoleId::new(role_id)?)
                        .await?,
                );
            }
        }

        Ok(SearchResponse {
            users: users.iter().map(UserDto::from).collect(),
        })
    }
}
