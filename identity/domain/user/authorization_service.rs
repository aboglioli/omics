use std::sync::Arc;

use common::error::Error;
use common::result::Result;

use crate::domain::token::{Token, TokenService};
use crate::domain::user::{User, UserId, UserRepository};

pub struct AuthorizationService {
    user_repo: Arc<dyn UserRepository>,

    token_serv: Arc<TokenService>,
}

impl AuthorizationService {
    pub fn new(user_repo: Arc<dyn UserRepository>, token_serv: Arc<TokenService>) -> Self {
        AuthorizationService {
            user_repo,
            token_serv,
        }
    }

    pub async fn authorize(&self, token: &Token) -> Result<User> {
        let data = self.token_serv.validate(token).await?;
        if let Some(user_id) = data.get("user_id") {
            let user_id = UserId::new(user_id)?;
            let user = self.user_repo.find_by_id(&user_id).await?;
            return Ok(user);
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

        let auth_user = serv.authorize(&token).await.unwrap();
        assert_eq!(auth_user.base(), user.base());

        assert!(serv.authorize(&Token::new("invalid")).await.is_err());
    }
}
