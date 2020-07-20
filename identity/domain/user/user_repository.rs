use crate::common::error::Error;
use crate::identity::domain::user::{User, UserID};

pub trait UserRepository {
    fn next_id(&self) -> Result<UserID, Error>;
    fn find_by_id(&self, id: UserID) -> Result<User, Error>;
    fn find_by_username_or_email(&self, username_or_email: &str) -> Result<User, Error>;
    fn save(&self, user: &mut User) -> Result<(), Error>;
}
