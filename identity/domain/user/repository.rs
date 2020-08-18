use async_trait::async_trait;

use common::result::Result;

use crate::domain::user::{Email, User, UserId, Username};

#[async_trait]
pub trait UserRepository {
    async fn next_id(&self) -> Result<UserId>;

    async fn find_all(&self) -> Result<Vec<User>>;
    async fn find_by_id(&self, id: &UserId) -> Result<User>;
    async fn find_by_username(&self, username: &Username) -> Result<User>;
    async fn find_by_email(&self, email: &Email) -> Result<User>;

    async fn save(&self, user: &mut User) -> Result<()>;
}
