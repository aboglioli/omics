use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

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
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> Result<Vec<Author>>;

    async fn save(&self, author: &mut Author) -> Result<()>;

    async fn delete(&self, id: &AuthorId) -> Result<()>;
}
