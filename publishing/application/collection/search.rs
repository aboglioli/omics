use serde::{Deserialize, Serialize};

use common::request::Include;
use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, CollectionDto};
use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::category::{CategoryId, CategoryRepository};
use crate::domain::collection::CollectionRepository;
use crate::domain::publication::PublicationId;

#[derive(Deserialize)]
pub struct SearchCommand {
    pub author_id: Option<String>,
    pub category_id: Option<String>,
    pub publication_id: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    collections: Vec<CollectionDto>,
}

pub struct Search<'a> {
    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    collection_repo: &'a dyn CollectionRepository,
}

impl<'a> Search<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        collection_repo: &'a dyn CollectionRepository,
    ) -> Self {
        Search {
            author_repo,
            category_repo,
            collection_repo,
        }
    }

    pub async fn exec(
        &self,
        _auth_id: Option<String>,
        cmd: SearchCommand,
        include: Include,
    ) -> Result<SearchResponse> {
        let collections = self
            .collection_repo
            .search(
                cmd.author_id.map(AuthorId::new).transpose()?.as_ref(),
                cmd.category_id.map(CategoryId::new).transpose()?.as_ref(),
                cmd.publication_id
                    .map(PublicationId::new)
                    .transpose()?
                    .as_ref(),
                cmd.name.as_ref(),
            )
            .await?;

        let mut collection_dtos = Vec::new();

        for collection in collections.iter() {
            let mut collection_dto = CollectionDto::from(collection);

            if include.has("author") {
                let author = self.author_repo.find_by_id(collection.author_id()).await?;
                collection_dto = collection_dto.author(AuthorDto::from(&author));
            }

            if include.has("category") {
                let category = self
                    .category_repo
                    .find_by_id(collection.header().category_id())
                    .await?;
                collection_dto = collection_dto.category(CategoryDto::from(&category));
            }

            collection_dtos.push(collection_dto);
        }

        Ok(SearchResponse {
            collections: collection_dtos,
        })
    }
}
