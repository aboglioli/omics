use common::error::Error;
use common::result::Result;

use crate::application::util;
use crate::domain::user::{User, UserId, UserRepository};

pub struct Delete<'a, URepo> {
    #[allow(dead_code)]
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
        util::is_authorized(auth_user, user_id)?;

        Err(Error::new("user", "not_implemented_yet"))
    }
}
