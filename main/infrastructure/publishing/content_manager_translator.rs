use std::sync::Arc;

use async_trait::async_trait;

use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use publishing::domain::content_manager::{
    ContentManager, ContentManagerId, ContentManagerRepository,
};

// TODO: use role
pub struct ContentManagerTranslator<URepo> {
    user_repo: Arc<URepo>,
}

impl<URepo> ContentManagerTranslator<URepo> {
    pub fn new(user_repo: Arc<URepo>) -> Self {
        ContentManagerTranslator { user_repo }
    }
}

#[async_trait]
impl<URepo> ContentManagerRepository for ContentManagerTranslator<URepo>
where
    URepo: UserRepository + Sync + Send,
{
    async fn next_id(&self) -> Result<ContentManagerId> {
        let user_id = self.user_repo.next_id().await?;
        Ok(ContentManagerId::new(user_id.value())?)
    }

    async fn find_by_id(&self, id: &ContentManagerId) -> Result<ContentManager> {
        let user = self.user_repo.find_by_id(&UserId::new(id.value())?).await?;

        Ok(ContentManager::new(ContentManagerId::new(
            user.base().id().value(),
        )?)?)
    }

    async fn save(&self, _author: &mut ContentManager) -> Result<()> {
        Ok(())
    }
}
