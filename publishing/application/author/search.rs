use serde::{Deserialize, Serialize};

use common::result::Result;
use shared::domain::user::UserRepository;

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
    user_repo: &'a dyn UserRepository,
}
impl<'a> Search<'a> {
    pub fn new(author_repo: &'a dyn AuthorRepository, user_repo: &'a dyn UserRepository) -> Self {
        Search {
            author_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        _auth_id: Option<String>,
        cmd: SearchCommand,
    ) -> Result<SearchResponse> {
        let users = self.user_repo.search(cmd.name.as_ref()).await?;

        let mut author_dtos = Vec::new();
        for user in users.iter() {
            let author = self.author_repo.find_by_id(user.base().id()).await?;
            author_dtos.push(AuthorDto::from(&user, &author));
        }

        Ok(SearchResponse {
            authors: author_dtos,
        })
    }
}
