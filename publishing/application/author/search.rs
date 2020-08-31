use serde::{Deserialize, Serialize};

use common::result::Result;

use crate::application::dtos::AuthorDto;
use crate::domain::author::AuthorRepository;

#[derive(Deserialize)]
pub struct SearchCommand {
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    authors: Vec<AuthorDto>,
}

pub struct Search<'a> {
    author_repo: &'a dyn AuthorRepository,
}
impl<'a> Search<'a> {
    pub fn new(author_repo: &'a dyn AuthorRepository) -> Self {
        Search { author_repo }
    }

    pub async fn exec(
        &self,
        _auth_id: Option<String>,
        cmd: SearchCommand,
    ) -> Result<SearchResponse> {
        let authors = self.author_repo.search(cmd.name.as_ref()).await?;

        Ok(SearchResponse {
            authors: authors.iter().map(AuthorDto::from).collect(),
        })
    }
}
