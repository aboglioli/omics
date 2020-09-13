use async_trait::async_trait;
use uuid::Uuid;

use common::result::Result;

use crate::domain::user::{User, UserId};

#[async_trait]
pub trait UserRepository: Sync + Send {
    async fn next_id(&self) -> Result<UserId> {
        UserId::new(Uuid::new_v4().to_string())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<User>;
    async fn search(&self, name: Option<&String>) -> Result<Vec<User>>;

    async fn save(&self, user: &mut User) -> Result<()>;
}
