use serde::Serialize;

use common::request::Include;
use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, CollectionDto, PublicationDto};
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::{CollectionId, CollectionRepository};
use crate::domain::user::UserRepository;

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
    user_repo: &'a dyn UserRepository,
}

impl<'a> GetById<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        collection_repo: &'a dyn CollectionRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        GetById {
            author_repo,
            category_repo,
            collection_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        _auth_id: Option<String>,
        collection_id: String,
        include: Include,
    ) -> Result<CollectionDto> {
        let collection = self
            .collection_repo
            .find_by_id(&CollectionId::new(collection_id)?)
            .await?;
        let mut collection_dto = CollectionDto::from(&collection);

        if include.has("author") {
            let user = self.user_repo.find_by_id(collection.author_id()).await?;
            let author = self.author_repo.find_by_id(collection.author_id()).await?;
            collection_dto = collection_dto.author(AuthorDto::from(&user, &author));
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
