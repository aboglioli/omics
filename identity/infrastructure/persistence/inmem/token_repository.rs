use async_trait::async_trait;

use common::cache::Cache;
use common::infrastructure::cache::InMemCache;
use common::result::Result;

use crate::domain::token::{Data, TokenId, TokenRepository};

#[derive(Default)]
pub struct InMemTokenRepository {
    cache: InMemCache<TokenId, Data>,
}

impl InMemTokenRepository {
    pub fn new() -> Self {
        InMemTokenRepository {
            cache: InMemCache::new(),
        }
    }

    pub fn cache(&self) -> &InMemCache<TokenId, Data> {
        &self.cache
    }
}

#[async_trait]
impl Cache<TokenId, Data> for InMemTokenRepository {
    async fn get(&self, token_id: &TokenId) -> Option<Data> {
        self.cache.get(token_id).await
    }

    async fn set(&self, token_id: TokenId, data: Data) -> Result<()> {
        self.cache.set(token_id, data).await
    }

    async fn delete(&self, token_id: &TokenId) -> Result<()> {
        self.cache.delete(token_id).await
    }
}

impl TokenRepository for InMemTokenRepository {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        fn check_trait_impl<T: TokenRepository>(_repo: &T) {}

        let repo = InMemTokenRepository::new();
        check_trait_impl(&repo);

        let mut data = Data::new();
        data.add("user_id", "U002");

        repo.set(TokenId::from("T123"), data.clone()).await.unwrap();
        repo.set(TokenId::from("T124"), data.clone()).await.unwrap();

        let saved_data = repo.get(&TokenId::from("T123")).await.unwrap();
        assert!(saved_data.get("user_id").is_some());
        assert_eq!(data.get("user_id"), saved_data.get("user_id"));

        assert!(repo.get(&TokenId::from("T777")).await.is_none());
        assert!(repo.get(&TokenId::from("T123")).await.is_some());

        assert!(repo.delete(&TokenId::from("T123")).await.is_ok());
        assert!(repo.get(&TokenId::from("T124")).await.is_some());
    }
}
