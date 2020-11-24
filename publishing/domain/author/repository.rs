use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use common::error::Error;
use common::model::Pagination;
use common::result::Result;

use crate::domain::author::{Author, AuthorId};

#[async_trait]
pub trait AuthorRepository: Sync + Send {
    async fn next_id(&self) -> Result<AuthorId> {
        AuthorId::new(Uuid::new_v4().to_string())
    }

    async fn find_by_id(&self, id: &AuthorId) -> Result<Author>;
    async fn search(
        &self,
        name: Option<&String>,
        publications_gt: Option<u32>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
        order_by: Option<&AuthorOrderBy>,
    ) -> Result<Pagination<Author>>;

    async fn save(&self, author: &mut Author) -> Result<()>;

    async fn delete(&self, id: &AuthorId) -> Result<()>;
}

pub enum AuthorOrderBy {
    Oldest,
    Newest,
    Followers,
    Publications,
}

impl FromStr for AuthorOrderBy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "newest" => AuthorOrderBy::Newest,
            "followers" => AuthorOrderBy::Followers,
            "publications" => AuthorOrderBy::Publications,
            _ => AuthorOrderBy::Oldest,
        })
    }
}
