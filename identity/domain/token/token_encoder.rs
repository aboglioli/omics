use crate::domain::token::{Token, TokenId};

use common::result::Result;

pub trait TokenEncoder {
    fn encode(&self, token_id: &TokenId) -> Result<Token>;
    fn decode(&self, token: &Token) -> Result<TokenId>;
}
