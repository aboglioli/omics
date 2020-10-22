use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use common::error::Error;
use common::model::Pagination;
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
    async fn search(
        &self,
        role_id: Option<&RoleId>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
        order_by: Option<&UserOrderBy>,
    ) -> Result<Pagination<User>>;

    async fn save(&self, user: &mut User) -> Result<()>;

    async fn delete(&self, id: &UserId) -> Result<()>;
}

pub enum UserOrderBy {
    Oldest,
    Newest,
}

impl FromStr for UserOrderBy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "newest" => UserOrderBy::Newest,
            _ => UserOrderBy::Oldest,
        })
    }
}
