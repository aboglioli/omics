use async_trait::async_trait;

use common::error::Error;

use crate::domain::author::{Author, AuthorId};

#[async_trait]
pub trait AuthorRepository {
    async fn find_by_id(&self, id: &AuthorId) -> Result<Author, Error>;
    async fn save(&self, author: &mut Author) -> Result<(), Error>;
}
