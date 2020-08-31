use common::result::Result;

use crate::application::dtos::AuthorDto;
use crate::domain::author::{AuthorId, AuthorRepository};

pub struct GetById<'a> {
    author_repo: &'a dyn AuthorRepository,
}

impl<'a> GetById<'a> {
    pub fn new(author_repo: &'a dyn AuthorRepository) -> Self {
        GetById { author_repo }
    }

    pub async fn exec(&self, _auth_id: Option<String>, author_id: String) -> Result<AuthorDto> {
        let author_id = AuthorId::new(author_id)?;
        let author = self.author_repo.find_by_id(&author_id).await?;

        Ok(AuthorDto::from(&author))
    }
}
