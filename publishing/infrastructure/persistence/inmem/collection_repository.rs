use async_trait::async_trait;
use chrono::{DateTime, Utc};

use common::cache::Cache;
use common::error::Error;
use common::infrastructure::cache::InMemCache;
use common::model::Pagination;
use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::collection::{
    Collection, CollectionId, CollectionOrderBy, CollectionRepository,
};
use crate::domain::publication::{PublicationId, Tag};

pub struct InMemCollectionRepository {
    cache: InMemCache<CollectionId, Collection>,
}

impl InMemCollectionRepository {
    pub fn new() -> Self {
        InMemCollectionRepository {
            cache: InMemCache::new(),
        }
    }
}

impl Default for InMemCollectionRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CollectionRepository for InMemCollectionRepository {
    async fn find_by_id(&self, id: &CollectionId) -> Result<Collection> {
        self.cache
            .get(id)
            .await
            .ok_or_else(|| Error::not_found("collection"))
    }

    async fn search(
        &self,
        author_id: Option<&AuthorId>,
        category_id: Option<&CategoryId>,
        publication_id: Option<&PublicationId>,
        _tag: Option<&Tag>,
        name: Option<&String>,
        _from: Option<&DateTime<Utc>>,
        _to: Option<&DateTime<Utc>>,
        _offset: Option<usize>,
        _limit: Option<usize>,
        _order_by: Option<&CollectionOrderBy>,
    ) -> Result<Pagination<Collection>> {
        let mut collections = self.cache.all().await;

        if let Some(author_id) = author_id {
            collections = collections
                .into_iter()
                .filter(|collection| collection.author_id() == author_id)
                .collect();
        }

        if let Some(category_id) = category_id {
            collections = collections
                .into_iter()
                .filter(|collection| collection.header().category_id() == category_id)
                .collect();
        }

        if let Some(publication_id) = publication_id {
            collections = collections
                .into_iter()
                .filter(|collection| {
                    for item in collection.items() {
                        if item.publication_id() == publication_id {
                            return true;
                        }
                    }

                    false
                })
                .collect();
        }

        if let Some(name) = name {
            collections = collections
                .into_iter()
                .filter(|collection| collection.header().name().to_string().contains(name))
                .collect();
        }

        Ok(Pagination::new(
            Some(0),
            Some(collections.len()),
            collections.len(),
            collections.len(),
        )
        .add_items(collections))
    }

    async fn save(&self, collection: &mut Collection) -> Result<()> {
        if collection.base().deleted_at().is_none() {
            self.cache
                .set(collection.base().id().clone(), collection.clone())
                .await
        } else {
            self.cache.delete(collection.base().id()).await
        }
    }

    async fn delete(&self, id: &CollectionId) -> Result<()> {
        self.cache.delete(id).await
    }
}
