use serde::{Deserialize, Serialize};

use common::result::Result;

use crate::application::dtos::CollectionDto;
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::CollectionRepository;
use crate::domain::publication::PublicationRepository;

#[derive(Deserialize)]
pub struct SearchCommand {
    author_id: Option<String>,
    category_id: Option<String>,
    status: Option<String>,
    text: Option<String>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    collections: Vec<CollectionDto>,
}

pub struct Search<'a, ARepo, CatRepo, CollRepo, PRepo> {
    author_repo: &'a ARepo,
    category_repo: &'a CatRepo,
    collection_repo: &'a CollRepo,
    publication_repo: &'a PRepo,
}

impl<'a, ARepo, CatRepo, CollRepo, PRepo> Search<'a, ARepo, CatRepo, CollRepo, PRepo>
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
        Search {
            author_repo,
            category_repo,
            collection_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self, _user_id: String, _cmd: SearchCommand) -> Result<SearchResponse> {
        Ok(SearchResponse {
            collections: Vec::new(),
        })
    }
}
