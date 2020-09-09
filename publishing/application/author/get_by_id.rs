use common::result::Result;

use crate::application::dtos::AuthorDto;
use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::user::UserRepository;

pub struct GetById<'a> {
    author_repo: &'a dyn AuthorRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> GetById<'a> {
    pub fn new(author_repo: &'a dyn AuthorRepository, user_repo: &'a dyn UserRepository) -> Self {
        GetById {
            author_repo,
            user_repo,
        }
    }

    pub async fn exec(&self, _auth_id: Option<String>, author_id: String) -> Result<AuthorDto> {
        let author_id = AuthorId::new(author_id)?;
        let author = self.author_repo.find_by_id(&author_id).await?;
        let user = self.user_repo.find_by_id(&author_id).await?;

        Ok(AuthorDto::from(&user, &author))
    }
}
