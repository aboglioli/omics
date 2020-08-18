use std::sync::Arc;

use async_trait::async_trait;

use common::error::Error;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use publishing::domain::content_manager::{
    ContentManager, ContentManagerId, ContentManagerRepository,
};

pub struct ContentManagerTranslator {
    user_repo: Arc<dyn UserRepository>,
}

impl ContentManagerTranslator {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        ContentManagerTranslator { user_repo }
    }
}

#[async_trait]
impl ContentManagerRepository for ContentManagerTranslator {
    async fn next_id(&self) -> Result<ContentManagerId> {
        let user_id = self.user_repo.next_id().await?;
        Ok(ContentManagerId::new(user_id.value())?)
    }

    async fn find_by_id(&self, id: &ContentManagerId) -> Result<ContentManager> {
        let user = self.user_repo.find_by_id(&UserId::new(id.value())?).await?;

        if !user.role().is("admin") && !user.role().is("content-manager") {
            return Err(Error::new("user", "unauthorized"));
        }

        Ok(ContentManager::new(ContentManagerId::new(
            user.base().id().value(),
        )?)?)
    }

    async fn save(&self, _author: &mut ContentManager) -> Result<()> {
        Ok(())
    }
}
