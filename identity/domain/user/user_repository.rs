use crate::domain::user::{Email, User, UserId, Username};
use common::error::Error;

pub trait UserRepository {
    fn next_id(&self) -> Result<UserId, Error>;
    fn find_by_id(&self, id: &UserId) -> Result<User, Error>;
    fn find_by_username(&self, username: &Username) -> Result<User, Error>;
    fn find_by_email(&self, email: &Email) -> Result<User, Error>;
    fn save(&self, user: &mut User) -> Result<(), Error>;
}
