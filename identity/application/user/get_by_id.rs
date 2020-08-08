use serde::Serialize;

use common::error::Error;
use common::result::Result;

use crate::domain::user::{User, UserId, UserRepository};

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

    pub async fn exec(&self, auth_user: &User, user_id: &UserId) -> Result<GetByIdResponse> {
        authorized(auth_user, user_id)?;

        let user = self.user_repo.find_by_id(user_id).await?;
        Ok(GetByIdResponse {
            username: user.identity().username().value().to_owned(),
        })
    }
}

fn authorized(auth_user: &User, user_id: &UserId) -> Result<()> {
    let guard = &auth_user.base().id() == user_id || auth_user.role().base().id() == "admin";

    if !guard {
        return Err(Error::new("user", "unauthorized"));
    }

    Ok(())
}
