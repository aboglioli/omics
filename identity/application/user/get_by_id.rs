use serde::Serialize;

use common::error::Error;

use crate::domain::user::{UserId, UserRepository};

#[derive(Serialize)]
pub struct GetByIdResponse {
    username: String,
}

pub struct GetById<'a, URepo> {
    user_repo: &'a URepo,
}

impl<'a, URepo> GetById<'a, URepo>
where
    URepo: UserRepository,
{
    pub fn new(user_repo: &'a URepo) -> Self {
        GetById { user_repo }
    }

    pub async fn exec(&self, user_id: &UserId) -> Result<GetByIdResponse, Error> {
        let user = self.user_repo.find_by_id(user_id).await?;
        Ok(GetByIdResponse {
            username: user.identity().username().value().to_owned(),
        })
    }
}
