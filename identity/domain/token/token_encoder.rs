use crate::domain::token::{Token, TokenID};
use common::error::Error;

pub trait TokenEncoder {
    fn encode(&self, token_id: &TokenID) -> Result<Token, Error>;
    fn decode(&self, token: Token) -> Result<TokenID, Error>;
}
