use serde::Serialize;

use common::error::Error;
use common::request::Include;
use common::result::Result;
use identity::UserIdAndRole;

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
        user_id_and_role: Option<UserIdAndRole>,
        collection_id: String,
        include: Include,
    ) -> Result<GetPublicationsResponse> {
        let collection = self
            .collection_repo
            .find_by_id(&CollectionId::new(collection_id)?)
            .await?;

        let can_view_unpublished_publications = if let Some((auth_id, auth_role)) = user_id_and_role
        {
            if !auth_role.can("get_publications_from_collection") {
                return Err(Error::unauthorized());
            }

            collection.author_id() == &auth_id || auth_role.can("get_unpublished_publications")
        } else {
            false
        };

        let mut publication_dtos = Vec::new();

        for item in collection.items() {
            let publication = self
                .publication_repo
                .find_by_id(item.publication_id())
                .await?;

            if !can_view_unpublished_publications && !publication.is_published() {
                continue;
            }

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
