use std::sync::Arc;

use warp::{Filter, Rejection};

use common::error::{public::PublicError, Error};
use common::result::Result;
use identity::domain::token::Token;

use crate::container::{with_container, Container};

fn extract_token<S: Into<String>>(authorization: S) -> Result<Token> {
    let authorization = authorization.into();

    if authorization.starts_with("Bearer ") {
        if let Some(token) = authorization.strip_prefix("Bearer ") {
            return Ok(Token::new(token));
        }
    }

    Err(Error::new("authorization", "invalid_header")
        .set_status(401)
        .set_message("Authorization header is not present")
        .build())
}

async fn with_user<S: Into<String>>(authorization_header: S, c: &Container) -> Result<String> {
    let authorization_header = authorization_header.into();

    let token = extract_token(authorization_header)?;
    let authorization_serv = c.identity.authorization_serv();
    if let Ok(user) = authorization_serv.authorize(&token).await {
        return Ok(user.base().id().value().to_owned());
    }

    Err(Error::new("authorization", "unauthorized")
        .set_status(401)
        .set_message("User is not logged in")
        .build())
}

pub fn with_auth(c: Arc<Container>) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::header::<String>("authorization")
        .and(with_container(c.clone()))
        .and_then(
            |authorization_header: String, c: Arc<Container>| async move {
                match with_user(authorization_header, &c).await {
                    Ok(user_id) => Ok(user_id),
                    Err(err) => {
                        let pub_err = PublicError::from(&err, false).unwrap();
                        Err(warp::reject::custom(pub_err))
                    }
                }
            },
        )
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
