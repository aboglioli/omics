use serde::Serialize;

use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, CollectionDto, PublicationDto};
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::{CollectionId, CollectionRepository};
use crate::domain::publication::PublicationRepository;

#[derive(Serialize)]
pub struct GetByIdResponse {
    pub id: String,
    pub author: AuthorDto,
    pub name: String,
    pub synopsis: String,
    pub category: CategoryDto,
    pub tags: Vec<String>,
    pub publications: Vec<PublicationDto>,
}

pub struct GetById<'a> {
    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    collection_repo: &'a dyn CollectionRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> GetById<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        collection_repo: &'a dyn CollectionRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        GetById {
            author_repo,
            category_repo,
            collection_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self, collection_id: String) -> Result<CollectionDto> {
        let collection_id = CollectionId::new(collection_id)?;
        let collection = self.collection_repo.find_by_id(&collection_id).await?;

        let author = self.author_repo.find_by_id(collection.author_id()).await?;
        let category = self
            .category_repo
            .find_by_id(collection.header().category_id())
            .await?;

        let mut publications = Vec::new();
        for item in collection.items().iter() {
            let publication = self
                .publication_repo
                .find_by_id(item.publication_id())
                .await?;
            let author = self.author_repo.find_by_id(publication.author_id()).await?;
            let category = self
                .category_repo
                .find_by_id(publication.header().category_id())
                .await?;

            publications.push(PublicationDto::new(
                &publication,
                AuthorDto::new(&author),
                CategoryDto::new(&category),
                false,
                false,
            ));
        }

        Ok(CollectionDto::new(
            &collection,
            AuthorDto::new(&author),
            CategoryDto::new(&category),
            publications,
        ))
    }
}
