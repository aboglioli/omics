use serde::{Deserialize, Serialize};

use common::result::Result;

use crate::application::dtos::AuthorDto;
use crate::domain::author::AuthorRepository;
use crate::domain::collection::CollectionRepository;
use crate::domain::publication::PublicationRepository;

#[derive(Deserialize)]
pub struct SearchCommand {
    pub name: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub authors: Vec<AuthorDto>,
}

pub struct Search<'a> {
    author_repo: &'a dyn AuthorRepository,
    collection_repo: &'a dyn CollectionRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> Search<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        collection_repo: &'a dyn CollectionRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        Search {
            author_repo,
            collection_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self, cmd: SearchCommand) -> Result<SearchResponse> {
        let authors = self.author_repo.search(&cmd.name).await?;

        let mut author_dtos = Vec::new();
        for author in authors.iter() {
            author_dtos.push(
                AuthorDto::new(author)
                    .publication_count(
                        self.publication_repo
                            .find_by_author_id(&author.base().id())
                            .await?
                            .len(),
                    )
                    .collection_count(
                        self.collection_repo
                            .find_by_author_id(&author.base().id())
                            .await?
                            .len(),
                    ),
            )
        }

        Ok(SearchResponse {
            authors: author_dtos,
        })
    }
}
