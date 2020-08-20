use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use common::error::Error;
use common::result::Result;

use crate::domain::token::{Token, TokenEncoder, TokenId};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Default)]
pub struct JWTEncoder;

impl JWTEncoder {
    pub fn new() -> Self {
        JWTEncoder
    }
}

impl TokenEncoder for JWTEncoder {
    fn encode(&self, token_id: &TokenId) -> Result<Token> {
        let claims = Claims {
            sub: token_id.to_string(),
            company: "Omics".to_owned(),
            exp: 10000000000,
        };
        let key = b"secret";

        let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(key)) {
            Ok(token) => token,
            Err(err) => return Err(Error::new("token", "encode").wrap_raw(err).build()),
        };

        Ok(Token::new(token))
    }

    fn decode(&self, token: &Token) -> Result<TokenId> {
        let key = b"secret";

        let token_data = match decode::<Claims>(
            token.value(),
            &DecodingKey::from_secret(key),
            &Validation::default(),
        ) {
            Ok(data) => data,
            Err(err) => return Err(Error::new("token", "decode").wrap_raw(err).build()),
        };

        Ok(TokenId::build(token_data.claims.sub))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        let token_id = TokenId::build("#token01");
        let enc = JWTEncoder::new();

        let token = enc.encode(&token_id).unwrap();
        assert!(token.value().len() > 10);
        assert_eq!(enc.decode(&token).unwrap(), token_id);
    }
}
