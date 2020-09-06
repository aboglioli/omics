use std::sync::Arc;

use async_trait::async_trait;

use common::cache::Cache;
use common::infrastructure::cache::InMemCache;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use publishing::domain::reader::{Preferences, Reader, ReaderId, ReaderRepository};

pub struct ReaderTranslator {
    user_repo: Arc<dyn UserRepository>,
    preferences_cache: InMemCache<ReaderId, Preferences>,
}

impl ReaderTranslator {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        ReaderTranslator {
            user_repo,
            preferences_cache: InMemCache::new(),
        }
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
        let mut reader = Reader::new(
            ReaderId::new(user.base().id().value())?,
            user.identity().username().value(),
            user.person().unwrap().fullname().name(),
            user.person().unwrap().fullname().lastname(),
        )?;

        if let Some(preferences) = self.preferences_cache.get(id).await {
            reader.set_preferences(preferences.clone())?;
        }

        Ok(reader)
    }

    async fn save(&self, reader: &mut Reader) -> Result<()> {
        let preferences = reader.preferences().clone();
        self.preferences_cache
            .set(reader.base().id().clone(), preferences)
            .await?;

        Ok(())
    }
}
