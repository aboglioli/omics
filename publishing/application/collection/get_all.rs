use serde::Serialize;

use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, CollectionDto, PublicationDto};
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::CollectionRepository;
use crate::domain::publication::PublicationRepository;

#[derive(Serialize)]
pub struct GetAllResponse {
    collections: Vec<CollectionDto>,
}

pub struct GetAll<'a> {
    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    collection_repo: &'a dyn CollectionRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> GetAll<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        collection_repo: &'a dyn CollectionRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        GetAll {
            author_repo,
            category_repo,
            collection_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self) -> Result<GetAllResponse> {
        let collections = self.collection_repo.find_all().await?;

        let mut collection_dtos = Vec::new();
        for collection in collections.iter() {
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

                let publication_dto = PublicationDto::from(&publication)
                    .author(AuthorDto::from(&author))
                    .category(CategoryDto::from(&category))
                    .status(&publication);

                publications.push(publication_dto);
            }

            collection_dtos.push(
                CollectionDto::from(collection)
                    .author(AuthorDto::from(&author))
                    .category(CategoryDto::from(&category))
                    .publications(publications),
            )
        }

        Ok(GetAllResponse {
            collections: collection_dtos,
        })
    }
}
