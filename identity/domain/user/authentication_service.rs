use std::sync::Arc;

use common::error::Error;
use common::result::Result;

use crate::domain::token::{Data, Token, TokenService};
use crate::domain::user::{Email, PasswordHasher, User, UserRepository, Username};

pub struct AuthenticationService {
    user_repo: Arc<dyn UserRepository>,

    password_hasher: Arc<dyn PasswordHasher>,

    token_serv: Arc<TokenService>,
}

/// AutenticationService authenticate any user, validated or not.
impl AuthenticationService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        token_serv: Arc<TokenService>,
    ) -> Self {
        AuthenticationService {
            user_repo,
            password_hasher,
            token_serv,
        }
    }

    pub async fn authenticate(
        &self,
        username_or_email: &str,
        password: &str,
    ) -> Result<(User, Token)> {
        let err = Error::new("credentials", "invalid");

        let mut user = match (
            Username::new(username_or_email),
            Email::new(username_or_email),
        ) {
            (Ok(username), Err(_)) => self.user_repo.find_by_username(&username).await,
            (Err(_), Ok(email)) => self.user_repo.find_by_email(&email).await,
            _ => return Err(err),
        }?;

        let user_password = match user.identity().password() {
            Some(password) => password.value(),
            None => return Err(err),
        };

        if self.password_hasher.compare(user_password, password) {
            let mut data = Data::new();
            data.add("user_id", user.base().id().value());

            let token = match self.token_serv.create(data).await {
                Ok(token) => token,
                Err(e) => return Err(err.wrap(e)),
            };

            user.login(&token)?;

            return Ok((user, token));
        }
        Err(err)
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks;

    #[tokio::test]
    async fn authenticate() {
        let c = mocks::container();
        let serv = c.authentication_serv();

        let mut user = mocks::validated_user1();
        c.user_repo().save(&mut user).await.unwrap();

        let (_, token) = serv
            .authenticate(user.identity().username().value(), "P@asswd!")
            .await
            .unwrap();
        assert!(!token.value().is_empty());

        let (_, token) = serv
            .authenticate(user.identity().email().value(), "P@asswd!")
            .await
            .unwrap();
        assert!(!token.value().is_empty());

        assert!(serv.authenticate("user2", "user123").await.is_err());
        assert!(serv.authenticate("user1", "user124").await.is_err());
        assert!(serv
            .authenticate("user@email.com.ar", "user123")
            .await
            .is_err());
        assert!(serv
            .authenticate("user@email.com", "user124")
            .await
            .is_err());
        assert!(serv
            .authenticate(user.identity().username().value(), "invalid")
            .await
            .is_err());
        assert!(serv
            .authenticate(user.identity().email().value(), "invalid")
            .await
            .is_err());
    }
}
