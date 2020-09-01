use std::sync::Arc;

use async_trait::async_trait;

use common::error::Error;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use publishing::domain::author::{Author, AuthorId, AuthorRepository};

pub struct AuthorTranslator {
    user_repo: Arc<dyn UserRepository>,
}

impl AuthorTranslator {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        AuthorTranslator { user_repo }
    }
}

#[async_trait]
impl AuthorRepository for AuthorTranslator {
    async fn next_id(&self) -> Result<AuthorId> {
        let user_id = self.user_repo.next_id().await?;
        Ok(AuthorId::new(user_id.value())?)
    }

    async fn find_all(&self) -> Result<Vec<Author>> {
        let users = self.user_repo.find_all().await?;
        let users = users.iter().filter(|user| user.person().is_some());

        let mut authors = Vec::new();
        for user in users {
            let author_id = AuthorId::new(user.base().id().value())?;

            authors.push(Author::new(
                author_id,
                user.identity().username().value(),
                user.person().unwrap().fullname().name(),
                user.person().unwrap().fullname().lastname(),
            )?)
        }

        Ok(authors)
    }

    async fn find_by_id(&self, id: &AuthorId) -> Result<Author> {
        let user = self.user_repo.find_by_id(&UserId::new(id.value())?).await?;
        let author_id = AuthorId::new(user.base().id().value())?;

        if user.person().is_none() {
            return Err(Error::new("author", "does_not_have_a_name"));
        }

        Ok(Author::new(
            author_id,
            user.identity().username().value(),
            user.person().unwrap().fullname().name(),
            user.person().unwrap().fullname().lastname(),
        )?)
    }

    async fn search(&self, _name: Option<&String>) -> Result<Vec<Author>> {
        let mut authors = Vec::new();

        for user in self.user_repo.find_all().await? {
            println!("{:?}", user);
            if user.person().is_none() {
                continue;
            }

            authors.push(
                Author::new(
                    user.base().id().clone(),
                    user.identity().username().value(),
                    user.person().unwrap().fullname().name(),
                    user.person().unwrap().fullname().lastname(),
                )?
            );
        }

        Ok(authors)
    }

    async fn save(&self, _author: &mut Author) -> Result<()> {
        Ok(())
    }
}
