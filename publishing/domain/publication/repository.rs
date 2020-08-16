use async_trait::async_trait;

use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::publication::{Publication, PublicationId};

#[async_trait]
pub trait PublicationRepository {
    async fn next_id(&self) -> Result<PublicationId>;

    async fn find_by_id(&self, id: &PublicationId) -> Result<Publication>;
    async fn find_by_author_id(&self, author_id: &AuthorId) -> Result<Vec<Publication>>;

    async fn save(&self, publication: &mut Publication) -> Result<()>;
}
