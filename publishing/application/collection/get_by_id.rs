use serde::Serialize;

use common::request::Include;
use common::result::Result;
use identity::UserIdAndRole;
use common::error::Error;

use crate::application::dtos::{AuthorDto, CategoryDto, CollectionDto, PublicationDto};
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::{CollectionId, CollectionRepository};

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
}

impl<'a> GetById<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        collection_repo: &'a dyn CollectionRepository,
    ) -> Self {
        GetById {
            author_repo,
            category_repo,
            collection_repo,
        }
    }

    pub async fn exec(
        &self,
        user_id_and_role: Option<UserIdAndRole>,
        collection_id: String,
        include: Include,
    ) -> Result<CollectionDto> {
        if let Some((auth_id, auth_role)) = user_id_and_role {
            if !auth_role.can("get_collection") {
                return Err(Error::unauthorized());
            }
        }

        let collection = self
            .collection_repo
            .find_by_id(&CollectionId::new(collection_id)?)
            .await?;
        let mut collection_dto = CollectionDto::from(&collection);

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

        Ok(collection_dto)
    }
}
