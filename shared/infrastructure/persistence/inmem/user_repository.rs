use async_trait::async_trait;

use common::cache::Cache;
use common::error::Error;
use common::infrastructure::cache::InMemCache;
use common::result::Result;

use crate::domain::user::{User, UserId, UserRepository};

pub struct InMemUserRepository {
    cache: InMemCache<UserId, User>,
}

impl InMemUserRepository {
    pub fn new() -> Self {
        InMemUserRepository {
            cache: InMemCache::new(),
        }
    }
}

impl Default for InMemUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for InMemUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<User> {
        self.cache
            .get(id)
            .await
            .ok_or_else(|| Error::not_found("user"))
    }

    async fn search(&self, name: Option<&String>) -> Result<Vec<User>> {
        let mut users = self.cache.all().await;

        if let Some(name) = name {
            users = users
                .into_iter()
                .filter(|user| {
                    // TODO: improve search
                    user.username().contains(name)
                })
                .collect();
        }

        Ok(users)
    }

    async fn save(&self, user: &mut User) -> Result<()> {
        if user.base().deleted_at().is_none() {
            self.cache.set(user.base().id().clone(), user.clone()).await
        } else {
            self.cache.delete(user.base().id()).await
        }
    }
}
