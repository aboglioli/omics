use crate::common::error::Error;
use crate::identity::domain::user::{Email, User, UserID, Username};

pub trait UserRepository {
    fn find_by_id(&self, id: UserID) -> Result<User, Error>;
    fn find_by_username(&self, username: Username) -> Result<User, Error>;
    fn find_by_email(&self, email: Email) -> Result<User, Error>;
    fn save(&self, user: &mut User) -> Result<(), Error>;
}
