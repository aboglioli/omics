use std::cell::RefCell;
use std::collections::HashMap;

use crate::domain::token::{Data, TokenId, TokenRepository};
use common::cache::Cache;
use common::error::Error;

pub struct InMemTokenRepository {
    pub cache: RefCell<HashMap<TokenId, Data>>,
}

impl InMemTokenRepository {
    pub fn new() -> InMemTokenRepository {
        InMemTokenRepository {
            cache: RefCell::new(HashMap::new()),
        }
    }
}

impl Cache<TokenId, Data> for InMemTokenRepository {
    fn get(&self, token_id: &TokenId) -> Option<Data> {
        let cache = self.cache.borrow();
        cache.get(token_id).cloned()
    }

    fn set(&self, token_id: TokenId, data: Data) -> Result<(), Error> {
        let mut cache = self.cache.borrow_mut();
        cache.insert(token_id, data);
        Ok(())
    }

    fn delete(&self, token_id: &TokenId) -> Result<(), Error> {
        let mut cache = self.cache.borrow_mut();
        cache.remove(token_id);
        Ok(())
    }
}

impl TokenRepository for InMemTokenRepository {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Error> {
        fn check_trait_impl<T: TokenRepository>(_repo: &T) {}

        let repo = InMemTokenRepository::new();
        check_trait_impl(&repo);

        let mut data = Data::new();
        data.add("user_id", "U002");

        repo.set(TokenId::from("T123"), data.clone())?;
        repo.set(TokenId::from("T124"), data.clone())?;

        let saved_data = repo.get(&TokenId::from("T123")).ok_or(Error::internal())?;
        assert!(saved_data.get("user_id").is_some());
        assert_eq!(data.get("user_id"), saved_data.get("user_id"));

        assert!(repo.get(&TokenId::from("T777")).is_none());
        assert!(repo.get(&TokenId::from("T123")).is_some());

        assert!(repo.delete(&TokenId::from("T123")).is_ok());
        assert!(repo.get(&TokenId::from("T124")).is_some());

        Ok(())
    }
}
