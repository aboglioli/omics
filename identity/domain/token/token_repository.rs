use crate::domain::token::{Data, TokenId};
use common::cache::Cache;

// pub trait TokenRepository {
//     fn get(&self, token_id: TokenId) -> Result<Data, Error>;
//     fn set(&self, token_id: TokenId, data: Data) -> Result<(), Error>;
//     fn delete(&self, tokne_id: TokenId) -> Result<(), Error>;
// }

pub trait TokenRepository: Cache<TokenId, Data> {}
