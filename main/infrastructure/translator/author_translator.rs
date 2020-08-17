use async_trait::async_trait;

use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use publishing::domain::author::{Author, AuthorId, AuthorRepository};

pub struct AuthorTranslator<'a, URepo> {
    user_repo: &'a URepo,
}

impl<'a, URepo> AuthorTranslator<'a, URepo> {
    pub fn new(user_repo: &'a URepo) -> Self {
        AuthorTranslator { user_repo }
    }
}

#[async_trait]
impl<'a, URepo> AuthorRepository for AuthorTranslator<'a, URepo>
where
    URepo: UserRepository + Sync + Send,
{
    async fn next_id(&self) -> Result<AuthorId> {
        let user_id = self.user_repo.next_id().await?;
        Ok(AuthorId::new(user_id.value())?)
    }

    async fn find_by_id(&self, id: &AuthorId) -> Result<Author> {
        let user = self.user_repo.find_by_id(&UserId::new(id.value())?).await?;

        Ok(Author::new(
            AuthorId::new(user.base().id().value())?,
            user.identity().username().value(),
            user.person().unwrap().fullname().name(),
            user.person().unwrap().fullname().lastname(),
        )?)
    }

    async fn search(&self, _text: &str) -> Result<Vec<Author>> {
        Ok(Vec::new())
    }

    async fn save(&self, _author: &mut Author) -> Result<()> {
        Ok(())
    }
}
