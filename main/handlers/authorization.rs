use common::error::Error;
use common::result::Result;
use identity::domain::token::Token;

// TODO: test it
pub fn extract_token(authorization: &str) -> Result<Token> {
    if authorization.starts_with("Bearer ") {
        if let Some(token) = authorization.strip_prefix("Bearer ") {
            return Ok(Token::new(token));
        }
    }

    Err(Error::new("authorization", "invalid_header"))
}
