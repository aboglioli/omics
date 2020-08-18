use serde::Serialize;

use common::error::Error;
use common::result::Result;

use crate::application::dtos::UserDto;
use crate::domain::user::{UserId, UserRepository};

#[derive(Serialize)]
pub struct GetAllResponse {
    pub users: Vec<UserDto>,
}

pub struct GetAll<'a, URepo> {
    user_repo: &'a URepo,
}

impl<'a, URepo> GetAll<'a, URepo>
where
    URepo: UserRepository,
{
    pub fn new(user_repo: &'a URepo) -> Self {
        GetAll { user_repo }
    }

    pub async fn exec(&self, viewer_id: String) -> Result<GetAllResponse> {
        let user_id = UserId::new(viewer_id)?;
        let user = self.user_repo.find_by_id(&user_id).await?;

        if !user.role().is("admin") {
            return Err(Error::new("user", "unauthorized"));
        }

        let users = self.user_repo.find_all().await?;

        Ok(GetAllResponse {
            users: users.iter().map(|user| UserDto::new(user, true)).collect(),
        })
    }
}
