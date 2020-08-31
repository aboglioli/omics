use async_trait::async_trait;

use common::result::Result;

use crate::domain::author::{Author, AuthorId};

#[async_trait]
pub trait AuthorRepository: Sync + Send {
    async fn next_id(&self) -> Result<AuthorId>;

    async fn find_all(&self) -> Result<Vec<Author>>;
    async fn find_by_id(&self, id: &AuthorId) -> Result<Author>;
    async fn search(&self, name: Option<&String>) -> Result<Vec<Author>>;

    async fn save(&self, author: &mut Author) -> Result<()>;
}
