use common::error::Error;

use crate::domain::user::{User, UserID};

pub trait UserRepository {
    fn find_by_id(&self, id: &UserID) -> Result<User, Error>;
}
