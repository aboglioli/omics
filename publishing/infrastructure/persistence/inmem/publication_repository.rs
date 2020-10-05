use async_trait::async_trait;

use common::cache::Cache;
use common::error::Error;
use common::infrastructure::cache::InMemCache;
use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::publication::{Publication, PublicationId, PublicationRepository, Tag};

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
        _tag: Option<&Tag>,
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
        if publication.base().deleted_at().is_none() {
            self.cache
                .set(publication.base().id().clone(), publication.clone())
                .await
        } else {
            self.cache.delete(publication.base().id()).await
        }
    }

    async fn delete(&self, id: &PublicationId) -> Result<()> {
        self.cache.delete(id).await
    }
}
