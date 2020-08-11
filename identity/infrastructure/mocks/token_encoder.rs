use common::error::Error;
use common::result::Result;

use crate::domain::token::{Token, TokenEncoder, TokenId};

#[derive(Default)]
pub struct FakeTokenEncoder;

impl FakeTokenEncoder {
    pub fn new() -> Self {
        FakeTokenEncoder
    }
}

impl TokenEncoder for FakeTokenEncoder {
    fn encode(&self, token_id: &TokenId) -> Result<Token> {
        Ok(Token::new(&format!("<<token::{}", token_id.id())))
    }

    fn decode(&self, token: &Token) -> Result<TokenId> {
        if !token.token().starts_with("<<token::") {
            return Err(Error::internal("token_encoder", "cannot_decode"));
        }

        Ok(TokenId::from(token.token().split_at(9).1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let enc = FakeTokenEncoder::new();

        assert_eq!(
            enc.encode(&TokenId::from("t007"))?,
            Token::new("<<token::t007")
        );
        assert_eq!(
            enc.decode(&Token::new("<<token::t009x"))?,
            TokenId::from("t009x")
        );

        Ok(())
    }
}
