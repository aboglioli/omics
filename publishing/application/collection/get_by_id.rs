use serde::Serialize;

use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, CollectionDto, PublicationDto};
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::{CollectionId, CollectionRepository};
use crate::domain::publication::PublicationRepository;

#[derive(Serialize)]
pub struct GetByIdResponse {
    id: String,
    author: AuthorDto,
    name: String,
    synopsis: String,
    category: CategoryDto,
    tags: Vec<String>,
    publications: Vec<PublicationDto>,
}

pub struct GetById<'a, ARepo, CatRepo, CollRepo, PRepo> {
    author_repo: &'a ARepo,
    category_repo: &'a CatRepo,
    collection_repo: &'a CollRepo,
    publication_repo: &'a PRepo,
}

impl<'a, ARepo, CatRepo, CollRepo, PRepo> GetById<'a, ARepo, CatRepo, CollRepo, PRepo>
where
    ARepo: AuthorRepository,
    CatRepo: CategoryRepository,
    CollRepo: CollectionRepository,
    PRepo: PublicationRepository,
{
    pub fn new(
        author_repo: &'a ARepo,
        category_repo: &'a CatRepo,
        collection_repo: &'a CollRepo,
        publication_repo: &'a PRepo,
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
                AuthorDto::new(&author, None),
                CategoryDto::new(&category),
                false,
                false,
            ));
        }

        Ok(CollectionDto::new(
            &collection,
            AuthorDto::new(&author, None),
            CategoryDto::new(&category),
            publications,
        ))
    }
}
