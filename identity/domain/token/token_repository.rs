use crate::domain::token::{Data, TokenID};
use common::cache::Cache;

// pub trait TokenRepository {
//     fn get(&self, token_id: TokenID) -> Result<Data, Error>;
//     fn set(&self, token_id: TokenID, data: Data) -> Result<(), Error>;
//     fn delete(&self, tokne_id: TokenID) -> Result<(), Error>;
// }

pub trait TokenRepository: Cache<TokenID, Data> {}
