use std::sync::Arc;

use async_trait::async_trait;

use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use publishing::domain::reader::{Reader, ReaderId, ReaderRepository};

pub struct ReaderTranslator {
    user_repo: Arc<dyn UserRepository>,
}

impl ReaderTranslator {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        ReaderTranslator { user_repo }
    }
}

#[async_trait]
impl ReaderRepository for ReaderTranslator {
    async fn next_id(&self) -> Result<ReaderId> {
        let user_id = self.user_repo.next_id().await?;
        Ok(ReaderId::new(user_id.value())?)
    }

    async fn find_by_id(&self, id: &ReaderId) -> Result<Reader> {
        let user = self.user_repo.find_by_id(&UserId::new(id.value())?).await?;

        Ok(Reader::new(
            ReaderId::new(user.base().id().value())?,
            user.identity().username().value(),
            user.person().unwrap().fullname().name(),
            user.person().unwrap().fullname().lastname(),
        )?)
    }

    async fn save(&self, _author: &mut Reader) -> Result<()> {
        Ok(())
    }
}
