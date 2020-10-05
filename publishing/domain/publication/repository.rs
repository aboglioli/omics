use async_trait::async_trait;
use uuid::Uuid;

use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::publication::{Publication, PublicationId, Tag};

#[async_trait]
pub trait PublicationRepository: Sync + Send {
    async fn next_id(&self) -> Result<PublicationId> {
        PublicationId::new(Uuid::new_v4().to_string())
    }

    async fn find_by_id(&self, id: &PublicationId) -> Result<Publication>;
    async fn search(
        &self,
        author_id: Option<&AuthorId>,
        category_id: Option<&CategoryId>,
        tag: Option<&Tag>,
        status: Option<&String>,
        name: Option<&String>,
    ) -> Result<Vec<Publication>>;

    async fn save(&self, publication: &mut Publication) -> Result<()>;

    async fn delete(&self, id: &PublicationId) -> Result<()>;
}
