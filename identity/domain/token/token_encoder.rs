use crate::common::error::Error;
use crate::identity::domain::token::{Token, TokenID};

pub trait TokenEncoder {
    fn encode(&self, token_id: &TokenID) -> Result<Token, Error>;
    fn decode(&self, token: Token) -> Result<TokenID, Error>;
}
