use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::collection::{Collection, CollectionId};
use crate::domain::publication::{PublicationId, Tag};

#[async_trait]
pub trait CollectionRepository: Sync + Send {
    async fn next_id(&self) -> Result<CollectionId> {
        CollectionId::new(Uuid::new_v4().to_string())
    }

    async fn find_by_id(&self, id: &CollectionId) -> Result<Collection>;
    async fn search(
        &self,
        author_id: Option<&AuthorId>,
        category_id: Option<&CategoryId>,
        publication_id: Option<&PublicationId>,
        tag: Option<&Tag>,
        name: Option<&String>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> Result<Vec<Collection>>;

    async fn save(&self, collection: &mut Collection) -> Result<()>;

    async fn delete(&self, id: &CollectionId) -> Result<()>;
}
