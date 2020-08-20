use std::sync::Arc;

use async_trait::async_trait;

use common::result::Result;

use crate::domain::catalogue::{Author, Category, Collection, CollectionService};
use publishing::domain::author::AuthorRepository;
use publishing::domain::category::CategoryRepository;
use publishing::domain::collection::{CollectionId, CollectionRepository};

pub struct SyncCollectionService {
    author_repo: Arc<dyn AuthorRepository>,
    category_repo: Arc<dyn CategoryRepository>,
    collection_repo: Arc<dyn CollectionRepository>,
}

impl SyncCollectionService {
    pub fn new(
        author_repo: Arc<dyn AuthorRepository>,
        category_repo: Arc<dyn CategoryRepository>,
        collection_repo: Arc<dyn CollectionRepository>,
    ) -> Self {
        SyncCollectionService {
            author_repo,
            category_repo,
            collection_repo,
        }
    }
}

#[async_trait]
impl CollectionService for SyncCollectionService {
    async fn get_by_id(&self, id: &str) -> Result<Collection> {
        let collection_id = CollectionId::new(id)?;
        let collection = self.collection_repo.find_by_id(&collection_id).await?;
        let author = self.author_repo.find_by_id(collection.author_id()).await?;
        let category = self
            .category_repo
            .find_by_id(collection.header().category_id())
            .await?;

        Ok(Collection::new(
            collection.base().id().to_string(),
            Author::new(
                author.base().id().value(),
                author.username(),
                author.name(),
                author.lastname(),
                self.collection_repo
                    .find_by_author_id(&author.base().id())
                    .await?
                    .len(),
            )?,
            collection.header().name().to_string(),
            collection.header().synopsis().to_string(),
            Category::new(category.base().id().value(), category.name().value())?,
            collection
                .header()
                .tags()
                .iter()
                .map(|tag| tag.name().to_string())
                .collect(),
            collection.header().cover().url().to_string(),
            collection.items().len(),
        )?)
    }
}
