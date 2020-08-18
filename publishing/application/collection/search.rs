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

pub struct Search<'a> {
    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    collection_repo: &'a dyn CollectionRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> Search<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        collection_repo: &'a dyn CollectionRepository,
        publication_repo: &'a dyn PublicationRepository,
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
