use common::error::Error;
use common::result::Result;

use crate::domain::user::{UserId, UserRepository};

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

    pub async fn exec(&self, _user_id: &UserId) -> Result<()> {
        Err(Error::new("user", "not_implemented_yet"))
    }
}
