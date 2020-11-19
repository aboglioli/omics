use serde::Serialize;

use common::error::Error;
use common::result::Result;
use identity::UserIdAndRole;

use crate::application::dtos::AuthorDto;
use crate::domain::author::AuthorRepository;
use crate::domain::interaction::InteractionRepository;
use crate::domain::reader::ReaderId;

#[derive(Serialize)]
pub struct GetFollowingResponse {
    authors: Vec<AuthorDto>,
}

pub struct GetFollowing<'a> {
    author_repo: &'a dyn AuthorRepository,
    interaction_repo: &'a dyn InteractionRepository,
}

impl<'a> GetFollowing<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        interaction_repo: &'a dyn InteractionRepository,
    ) -> Self {
        GetFollowing {
            author_repo,
            interaction_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        reader_id: String,
    ) -> Result<GetFollowingResponse> {
        if auth_id.value() != reader_id || !auth_role.can("get_reader_following") {
            return Err(Error::unauthorized());
        }

        let follows = self
            .interaction_repo
            .find_follows(Some(&ReaderId::new(reader_id)?), None, None, None)
            .await?;

        let mut author_dtos = Vec::new();

        for follow in follows.iter() {
            let author = self
                .author_repo
                .find_by_id(follow.base().id().author_id())
                .await?;
            author_dtos.push(AuthorDto::from(&author));
        }

        Ok(GetFollowingResponse {
            authors: author_dtos,
        })
    }
}
