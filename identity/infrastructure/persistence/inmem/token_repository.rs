use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;

use common::cache::Cache;
use common::error::Error;

use crate::domain::token::{Data, TokenId, TokenRepository};

pub struct InMemTokenRepository {
    pub cache: Mutex<HashMap<TokenId, Data>>,
}

impl InMemTokenRepository {
    pub fn new() -> InMemTokenRepository {
        InMemTokenRepository {
            cache: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Cache<TokenId, Data> for InMemTokenRepository {
    async fn get(&self, token_id: &TokenId) -> Option<Data> {
        let cache = self.cache.lock().unwrap();
        cache.get(token_id).cloned()
    }

    async fn set(&self, token_id: TokenId, data: Data) -> Result<(), Error> {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(token_id, data);
        Ok(())
    }

    async fn delete(&self, token_id: &TokenId) -> Result<(), Error> {
        let mut cache = self.cache.lock().unwrap();
        cache.remove(token_id);
        Ok(())
    }
}

impl TokenRepository for InMemTokenRepository {}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//     fn test() -> Result<(), Error> {
//         fn check_trait_impl<T: TokenRepository>(_repo: &T) {}
//
//         let repo = InMemTokenRepository::new();
//         check_trait_impl(&repo);
//
//         let mut data = Data::new();
//         data.add("user_id", "U002");
//
//         repo.set(TokenId::from("T123"), data.clone())?;
//         repo.set(TokenId::from("T124"), data.clone())?;
//
//         let saved_data = repo.get(&TokenId::from("T123")).await.ok_or(Error::internal())?;
//         assert!(saved_data.get("user_id").is_some());
//         assert_eq!(data.get("user_id"), saved_data.get("user_id"));
//
//         assert!(repo.get(&TokenId::from("T777")).is_none());
//         assert!(repo.get(&TokenId::from("T123")).is_some());
//
//         assert!(repo.delete(&TokenId::from("T123")).is_ok());
//         assert!(repo.get(&TokenId::from("T124")).is_some());
//
//         Ok(())
//     }
// }
