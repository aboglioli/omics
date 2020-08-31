use serde::Serialize;

use common::request::Include;
use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, PublicationDto};
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::{CollectionId, CollectionRepository};
use crate::domain::publication::PublicationRepository;

#[derive(Serialize)]
pub struct GetPublicationsResponse {
    publications: Vec<PublicationDto>,
}

pub struct GetPublications<'a> {
    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    collection_repo: &'a dyn CollectionRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> GetPublications<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        collection_repo: &'a dyn CollectionRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        GetPublications {
            author_repo,
            category_repo,
            collection_repo,
            publication_repo,
        }
    }

    pub async fn exec(
        &self,
        _auth_id: Option<String>,
        collection_id: String,
        include: Include,
    ) -> Result<GetPublicationsResponse> {
        let collection = self
            .collection_repo
            .find_by_id(&CollectionId::new(collection_id)?)
            .await?;

        let mut publication_dtos = Vec::new();

        for item in collection.items() {
            let publication = self
                .publication_repo
                .find_by_id(item.publication_id())
                .await?;
            let mut publication_dto = PublicationDto::from(&publication);

            if include.has("author") {
                let author = self.author_repo.find_by_id(publication.author_id()).await?;
                publication_dto = publication_dto.author(AuthorDto::from(&author));
            }

            if include.has("category") {
                let category = self
                    .category_repo
                    .find_by_id(publication.header().category_id())
                    .await?;
                publication_dto = publication_dto.category(CategoryDto::from(&category));
            }

            publication_dtos.push(publication_dto)
        }

        Ok(GetPublicationsResponse {
            publications: publication_dtos,
        })
    }
}
