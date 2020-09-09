use async_trait::async_trait;

use common::result::Result;

use crate::domain::user::User;
use crate::domain::user::UserId;

#[async_trait]
pub trait UserService {
    async fn get_by_id(id: &UserId) -> Result<User>;
}
