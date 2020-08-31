use async_trait::async_trait;
use uuid::Uuid;

use common::cache::Cache;
use common::error::Error;
use common::infrastructure::cache::InMemCache;
use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
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

    async fn find_all(&self) -> Result<Vec<Publication>> {
        Ok(self.cache.all().await)
    }

    async fn find_by_id(&self, id: &PublicationId) -> Result<Publication> {
        self.cache
            .get(id)
            .await
            .ok_or_else(|| Error::not_found("publication"))
    }

    async fn search(
        &self,
        author_id: Option<&AuthorId>,
        category_id: Option<&CategoryId>,
        status: Option<&String>,
        name: Option<&String>,
    ) -> Result<Vec<Publication>> {
        let mut publications = self.cache.all().await;

        if let Some(author_id) = author_id {
            publications = publications
                .into_iter()
                .filter(|publication| publication.author_id() == author_id)
                .collect();
        }

        if let Some(category_id) = category_id {
            publications = publications
                .into_iter()
                .filter(|publication| publication.header().category_id() == category_id)
                .collect();
        }

        if let Some(status) = status {
            publications = publications
                .into_iter()
                .filter(|publication| &publication.status_history().current().to_string() == status)
                .collect();
        }

        if let Some(name) = name {
            publications = publications
                .into_iter()
                .filter(|publication| publication.header().name().value().contains(name))
                .collect();
        }

        Ok(publications)
    }

    async fn save(&self, publication: &mut Publication) -> Result<()> {
        self.cache
            .set(publication.base().id().clone(), publication.clone())
            .await
    }
}
