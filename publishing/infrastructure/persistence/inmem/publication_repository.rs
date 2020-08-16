use async_trait::async_trait;
use uuid::Uuid;

use common::cache::{inmem::InMemCache, Cache};
use common::error::Error;
use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::publication::{Publication, PublicationId, PublicationRepository};

pub struct InMemPublicationRepository {
    cache: InMemCache<PublicationId, Publication>,
}

impl InMemPublicationRepository {
    pub fn new() -> Self {
        InMemPublicationRepository {
            cache: InMemCache::new(),
        }
    }
}

impl Default for InMemPublicationRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PublicationRepository for InMemPublicationRepository {
    async fn next_id(&self) -> Result<PublicationId> {
        let id = Uuid::new_v4();
        PublicationId::new(id.to_string())
    }

    async fn find_by_id(&self, id: &PublicationId) -> Result<Publication> {
        self.cache
            .get(id)
            .await
            .ok_or(Error::new("publication", "not_found"))
    }

    async fn find_by_author_id(&self, author_id: &AuthorId) -> Result<Vec<Publication>> {
        Ok(self
            .cache
            .filter(|&(_, publication)| publication.author_id() == author_id)
            .await)
    }

    async fn save(&self, publication: &mut Publication) -> Result<()> {
        self.cache
            .set(publication.base().id(), publication.clone())
            .await
    }
}
