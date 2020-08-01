use async_trait::async_trait;

use common::error::Error;
use common::result::Result;

use crate::domain::user::{Email, User, UserId, Username};

#[async_trait]
pub trait UserRepository {
    fn err_not_found() -> Error {
        Error::internal()
            .set_path("user-repository")
            .set_code("not_found")
            .build()
    }

    async fn next_id(&self) -> Result<UserId>;

    async fn find_by_id(&self, id: &UserId) -> Result<User>;
    async fn find_by_username(&self, username: &Username) -> Result<User>;
    async fn find_by_email(&self, email: &Email) -> Result<User>;

    async fn save(&self, user: &mut User) -> Result<()>;
}
