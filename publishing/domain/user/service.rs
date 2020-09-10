use async_trait::async_trait;

use common::result::Result;

use crate::domain::user::User;
use crate::domain::user::UserId;

#[async_trait]
pub trait UserService: Sync + Send {
    async fn get_by_id(&self, id: &UserId) -> Result<User>;
}
