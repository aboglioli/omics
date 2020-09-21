use serde::Serialize;

use common::result::Result;

use crate::application::dtos::{AuthorDto, ReaderAuthorInteractionDto};
use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::interaction::InteractionRepository;
use crate::domain::reader::ReaderId;

#[derive(Serialize)]
pub struct GetByIdResponse {
    pub author: AuthorDto,
    pub reader: Option<ReaderAuthorInteractionDto>,
}

pub struct GetById<'a> {
    author_repo: &'a dyn AuthorRepository,
    interaction_repo: &'a dyn InteractionRepository,
}

impl<'a> GetById<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        interaction_repo: &'a dyn InteractionRepository,
    ) -> Self {
        GetById {
            author_repo,
            interaction_repo,
        }
    }

    pub async fn exec(
        &self,
        auth_id: Option<String>,
        author_id: String,
    ) -> Result<GetByIdResponse> {
        let author_id = AuthorId::new(author_id)?;
        let author = self.author_repo.find_by_id(&author_id).await?;

        let reader_interaction_dto = if let Some(auth_id) = auth_id {
            if auth_id != author_id.value() {
                Some(ReaderAuthorInteractionDto::new(
                    !self
                        .interaction_repo
                        .find_follows(Some(&ReaderId::new(auth_id)?), Some(&author_id), None, None)
                        .await?
                        .is_empty(),
                ))
            } else {
                None
            }
        } else {
            None
        };

        Ok(GetByIdResponse {
            author: AuthorDto::from(&author),
            reader: reader_interaction_dto,
        })
    }
}
