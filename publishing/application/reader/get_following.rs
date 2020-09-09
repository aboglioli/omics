use serde::Serialize;

use common::error::Error;
use common::result::Result;

use crate::application::dtos::AuthorDto;
use crate::domain::author::AuthorRepository;
use crate::domain::interaction::InteractionRepository;
use crate::domain::reader::ReaderId;
use crate::domain::user::UserRepository;

#[derive(Serialize)]
pub struct GetFollowingResponse {
    authors: Vec<AuthorDto>,
}

pub struct GetFollowing<'a> {
    author_repo: &'a dyn AuthorRepository,
    interaction_repo: &'a dyn InteractionRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> GetFollowing<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        interaction_repo: &'a dyn InteractionRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        GetFollowing {
            author_repo,
            interaction_repo,
            user_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, reader_id: String) -> Result<GetFollowingResponse> {
        if auth_id != reader_id {
            return Err(Error::unauthorized());
        }

        let follows = self
            .interaction_repo
            .find_follows(Some(&ReaderId::new(reader_id)?), None, None, None)
            .await?;

        let mut author_dtos = Vec::new();

        for follow in follows.iter() {
            let author = self.author_repo.find_by_id(follow.author_id()).await?;
            let user = self.user_repo.find_by_id(follow.author_id()).await?;
            author_dtos.push(AuthorDto::from(&user, &author));
        }

        Ok(GetFollowingResponse {
            authors: author_dtos,
        })
    }
}
