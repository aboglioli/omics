use serde::Serialize;

use common::result::Result;

use crate::application::dtos::AuthorDto;
use crate::domain::author::AuthorRepository;
use crate::domain::collection::CollectionRepository;
use crate::domain::publication::PublicationRepository;

#[derive(Serialize)]
pub struct GetAllResponse {
    pub authors: Vec<AuthorDto>,
}

pub struct GetAll<'a> {
    author_repo: &'a dyn AuthorRepository,
    collection_repo: &'a dyn CollectionRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> GetAll<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        collection_repo: &'a dyn CollectionRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        GetAll {
            author_repo,
            collection_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self) -> Result<GetAllResponse> {
        let authors = self.author_repo.find_all().await?;

        let mut author_dtos = Vec::new();
        for author in authors.iter() {
            let publication_count = self.publication_repo
                .find_by_author_id(&author.base().id())
                .await?
                .len();

            let collection_count = self.collection_repo
                .find_by_author_id(&author.base().id())
                .await?
                .len();

            if publication_count == 0 && collection_count == 0 {
                continue;
            }

            author_dtos.push(
                AuthorDto::new(author)
                    .publication_count(publication_count)
                    .collection_count(collection_count),
            )
        }

        Ok(GetAllResponse {
            authors: author_dtos,
        })
    }
}
