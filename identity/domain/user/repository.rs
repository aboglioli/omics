use async_trait::async_trait;
use uuid::Uuid;

use common::result::Result;

use crate::domain::role::RoleId;
use crate::domain::user::{Email, User, UserId, Username};

#[async_trait]
pub trait UserRepository: Sync + Send {
    async fn next_id(&self) -> Result<UserId> {
        UserId::new(Uuid::new_v4().to_string())
    }

    async fn find_all(&self) -> Result<Vec<User>>;
    async fn find_by_id(&self, id: &UserId) -> Result<User>;
    async fn find_by_username(&self, username: &Username) -> Result<User>;
    async fn find_by_email(&self, email: &Email) -> Result<User>;
    async fn find_by_role_id(&self, role_id: &RoleId) -> Result<Vec<User>>;

    async fn save(&self, user: &mut User) -> Result<()>;

    async fn delete(&self, id: &UserId) -> Result<()>;
}
