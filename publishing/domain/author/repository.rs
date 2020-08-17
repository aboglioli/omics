use async_trait::async_trait;

use common::result::Result;

use crate::domain::author::{Author, AuthorId};

#[async_trait]
pub trait AuthorRepository {
    async fn next_id(&self) -> Result<AuthorId>;

    async fn find_by_id(&self, id: &AuthorId) -> Result<Author>;
    async fn search(&self, text: &str) -> Result<Vec<Author>>;

    async fn save(&self, author: &mut Author) -> Result<()>;
}
