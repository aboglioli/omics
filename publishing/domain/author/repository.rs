use async_trait::async_trait;
use uuid::Uuid;

use common::result::Result;

use crate::domain::author::{Author, AuthorId};

#[async_trait]
pub trait AuthorRepository: Sync + Send {
    async fn next_id(&self) -> Result<AuthorId> {
        AuthorId::new(Uuid::new_v4().to_string())
    }

    async fn find_all(&self) -> Result<Vec<Author>>;
    async fn find_by_id(&self, id: &AuthorId) -> Result<Author>;

    async fn save(&self, author: &mut Author) -> Result<()>;

    async fn delete(&self, id: &AuthorId) -> Result<()>;
}
