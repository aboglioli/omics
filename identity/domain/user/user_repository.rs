use crate::domain::user::{User, UserId};
use common::error::Error;

pub trait UserRepository {
    fn next_id(&self) -> Result<UserId, Error>;
    fn find_by_id(&self, id: &UserId) -> Result<User, Error>;
    fn find_by_username_or_email(&self, username_or_email: &str) -> Result<User, Error>;
    fn save(&self, user: &mut User) -> Result<(), Error>;
}
