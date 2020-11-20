use actix_web::HttpRequest;

use common::error::Error;
use identity::domain::token::Token;
use identity::domain::user::UserId;
use identity::UserIdAndRole;

use crate::container::MainContainer;
use crate::error::PublicError;

pub async fn auth(req: &HttpRequest, c: &MainContainer) -> Result<UserIdAndRole, PublicError> {
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

    let user_id = c
        .identity
        .authorization_serv()
        .authorize(&token)
        .await
        .map_err(PublicError::from)?;

    let user_id = UserId::new(user_id)?;

    let role = c
        .identity
        .role_repo()
        .find_by_user_id(&user_id)
        .await
        .map_err(PublicError::from)?;

    Ok((user_id, role))
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
