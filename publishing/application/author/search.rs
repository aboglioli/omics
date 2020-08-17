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

pub struct Search<'a, ARepo, CRepo, PRepo> {
    author_repo: &'a ARepo,
    collection_repo: &'a CRepo,
    publication_repo: &'a PRepo,
}

impl<'a, ARepo, CRepo, PRepo> Search<'a, ARepo, CRepo, PRepo>
where
    ARepo: AuthorRepository,
    CRepo: CollectionRepository,
    PRepo: PublicationRepository,
{
    pub fn new(
        author_repo: &'a ARepo,
        collection_repo: &'a CRepo,
        publication_repo: &'a PRepo,
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
