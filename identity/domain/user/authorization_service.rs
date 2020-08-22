use std::sync::Arc;

use common::error::Error;
use common::result::Result;

use crate::domain::token::{Token, TokenService};

pub struct AuthorizationService {
    token_serv: Arc<TokenService>,
}

impl AuthorizationService {
    pub fn new(token_serv: Arc<TokenService>) -> Self {
        AuthorizationService { token_serv }
    }

    pub async fn authorize(&self, token: &Token) -> Result<String> {
        if let Ok(data) = self.token_serv.validate(token).await {
            if let Some(user_id) = data.get("user_id") {
                return Ok(user_id.to_string());
            }
        }
        Err(Error::new("authorization", "unauthorized")
            .set_status(401)
            .set_message("User is not logged in")
            .build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::token::Data;
    use crate::mocks;

    #[tokio::test]
    async fn authorize() {
        let c = mocks::container();

        let mut user = mocks::validated_user1();
        c.user_repo().save(&mut user).await.unwrap();

        let mut data = Data::new();
        data.add("user_id", user.base().id().value());
        let token = c.token_serv().create(data).await.unwrap();

        let serv = c.authorization_serv();

        let user_id = serv.authorize(&token).await.unwrap();
        assert_eq!(user_id, user.base().id().to_string());

        assert!(serv.authorize(&Token::new("invalid")).await.is_err());
    }
}
