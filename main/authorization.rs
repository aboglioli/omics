use actix_web::HttpRequest;

use common::error::Error;
use identity::domain::token::Token;

use crate::container::MainContainer;
use crate::error::PublicError;

pub async fn auth(req: &HttpRequest, c: &MainContainer) -> Result<String, PublicError> {
    let auth_header = match req.headers().get("authorization") {
        Some(header) => {
            if let Ok(header) = header.to_str() {
                Ok(header.to_owned())
            } else {
                Err(Error::unauthorized().set_message("Invalid header"))
            }
        }
        None => Err(Error::unauthorized().set_message("Header is not present")),
    }
    .map_err(PublicError::from)?;

    let token = extract_token(auth_header).map_err(PublicError::from)?;

    c.identity
        .authorization_serv()
        .authorize(&token)
        .await
        .map_err(PublicError::from)
}

fn extract_token<S: Into<String>>(authorization: S) -> Result<Token, Error> {
    let authorization = authorization.into();

    if authorization.starts_with("Bearer ") {
        if let Some(token) = authorization.strip_prefix("Bearer ") {
            return Ok(Token::new(token));
        }
    }

    Err(Error::new("authorization", "invalid_header")
        .set_status(401)
        .set_message("Authorization header is not present"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_token() {
        let token = extract_token("Bearer token#123").unwrap();
        assert_eq!(token.value(), "token#123");
    }

    #[test]
    fn invalid_token() {
        assert!(extract_token("token#123").is_err());
    }
}
