use common::error::Error;
use common::result::Result;

use crate::domain::user::{User, UserId, UserRepository};

pub struct Delete<'a, URepo> {
    user_repo: &'a URepo,
}

impl<'a, URepo> Delete<'a, URepo>
where
    URepo: UserRepository,
{
    pub fn new(user_repo: &'a URepo) -> Self {
        Delete { user_repo }
    }

    pub async fn exec(&self, auth_user: &User, user_id: &UserId) -> Result<()> {
        authorized(auth_user, user_id)?;

        Err(Error::new("user", "not_implemented_yet"))
    }
}

fn authorized(auth_user: &User, user_id: &UserId) -> Result<()> {
    let guard = &auth_user.base().id() == user_id || auth_user.role().base().id() == "admin";

    if !guard {
        return Err(Error::new("user", "unauthorized"));
    }

    Ok(())
}
