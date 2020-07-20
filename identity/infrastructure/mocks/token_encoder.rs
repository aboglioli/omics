use common::error::Error;
use crate::domain::token::{Token, TokenEncoder, TokenID};

pub struct FakeTokenEncoder;

impl FakeTokenEncoder {
    pub fn new() -> FakeTokenEncoder {
        FakeTokenEncoder
    }
}

impl TokenEncoder for FakeTokenEncoder {
    fn encode(&self, token_id: &TokenID) -> Result<Token, Error> {
        Ok(Token::new(&format!("<<token::{}", token_id.id())))
    }

    fn decode(&self, token: Token) -> Result<TokenID, Error> {
        if !token.token().starts_with("<<token::") {
            return Err(Error::internal());
        }

        Ok(TokenID::from(token.token().split_at(9).1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Error> {
        let enc = FakeTokenEncoder::new();

        assert_eq!(
            enc.encode(&TokenID::from("t007"))?,
            Token::new("<<token::t007")
        );
        assert_eq!(
            enc.decode(Token::new("<<token::t009x"))?,
            TokenID::from("t009x")
        );

        Ok(())
    }
}
