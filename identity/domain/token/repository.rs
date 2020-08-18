use async_trait::async_trait;

use common::cache::Cache;

use crate::domain::token::{Data, TokenId};

#[async_trait]
pub trait TokenRepository: Cache<TokenId, Data> + Sync + Send {}
