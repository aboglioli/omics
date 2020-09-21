use serde::Serialize;

use common::error::Error;
use common::request::Include;
use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, CollectionDto, PublicationDto};
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::CollectionRepository;
use crate::domain::interaction::InteractionRepository;
use crate::domain::publication::PublicationRepository;
use crate::domain::reader::ReaderId;

#[derive(Serialize)]
pub struct GetFavoritesResponse {
    publications: Vec<PublicationDto>,
    collections: Vec<CollectionDto>,
}

pub struct GetFavorites<'a> {
    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    collection_repo: &'a dyn CollectionRepository,
    interaction_repo: &'a dyn InteractionRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> GetFavorites<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        collection_repo: &'a dyn CollectionRepository,
        interaction_repo: &'a dyn InteractionRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        GetFavorites {
            author_repo,
            category_repo,
            collection_repo,
            interaction_repo,
            publication_repo,
        }
    }

    pub async fn exec(
        &self,
        auth_id: String,
        reader_id: String,
        include: Include,
    ) -> Result<GetFavoritesResponse> {
        if auth_id != reader_id {
            return Err(Error::unauthorized());
        }

        let publication_favorites = self
            .interaction_repo
            .find_publication_favorites(Some(&ReaderId::new(&reader_id)?), None, None, None)
            .await?;

        let collection_favorites = self
            .interaction_repo
            .find_collection_favorites(Some(&ReaderId::new(reader_id)?), None, None, None)
            .await?;

        let mut publication_dtos = Vec::new();
        for favorite in publication_favorites.iter() {
            let publication = self
                .publication_repo
                .find_by_id(favorite.base().id().publication_id())
                .await?;

            if !publication.is_published() {
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

            publication_dtos.push(publication_dto);
        }

        let mut collection_dtos = Vec::new();
        for favorite in collection_favorites.iter() {
            let collection = self
                .collection_repo
                .find_by_id(favorite.base().id().collection_id())
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

            collection_dtos.push(collection_dto);
        }

        Ok(GetFavoritesResponse {
            publications: publication_dtos,
            collections: collection_dtos,
        })
    }
}
