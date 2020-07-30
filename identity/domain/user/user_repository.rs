use async_trait::async_trait;

use common::error::Error;

use crate::domain::user::{Email, User, UserId, Username};

#[async_trait]
pub trait UserRepository {
    fn err_not_found() -> Error {
        Error::internal()
            .set_path("user-repository")
            .set_code("not_found")
            .build()
    }

    async fn next_id(&self) -> Result<UserId, Error>;
    async fn find_by_id(&self, id: &UserId) -> Result<User, Error>;
    async fn find_by_username(&self, username: &Username) -> Result<User, Error>;
    async fn find_by_email(&self, email: &Email) -> Result<User, Error>;
    async fn save(&self, user: &mut User) -> Result<(), Error>;
}
