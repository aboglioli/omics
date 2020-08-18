use async_trait::async_trait;

use common::result::Result;

use crate::domain::content_manager::{ContentManager, ContentManagerId};

#[async_trait]
pub trait ContentManagerRepository: Sync + Send {
    async fn next_id(&self) -> Result<ContentManagerId>;

    async fn find_by_id(&self, id: &ContentManagerId) -> Result<ContentManager>;

    async fn save(&self, content_manager: &mut ContentManager) -> Result<()>;
}
