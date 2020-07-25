use crate::domain::token::{Token, TokenId};
use common::error::Error;

pub trait TokenEncoder {
    fn encode(&self, token_id: &TokenId) -> Result<Token, Error>;
    fn decode(&self, token: &Token) -> Result<TokenId, Error>;
}
