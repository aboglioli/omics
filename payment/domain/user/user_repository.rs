use common::error::Error;

use crate::domain::user::{User, UserId};

pub trait UserRepository {
    fn find_by_id(&self, id: &UserId) -> Result<User, Error>;
}
