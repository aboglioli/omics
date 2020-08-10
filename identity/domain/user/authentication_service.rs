use common::error::Error;
use common::result::Result;

use crate::domain::token::{Data, Token, TokenEncoder, TokenRepository, TokenService};
use crate::domain::user::{Email, PasswordHasher, User, UserRepository, Username};

pub struct AuthenticationService<'a, URepo, PHasher, TRepo, TEnc> {
    user_repo: &'a URepo,

    password_hasher: &'a PHasher,

    token_serv: TokenService<'a, TRepo, TEnc>,
}

impl<'a, URepo, PHasher, TRepo, TEnc> AuthenticationService<'a, URepo, PHasher, TRepo, TEnc>
where
    URepo: UserRepository,
    PHasher: PasswordHasher,
    TRepo: TokenRepository,
    TEnc: TokenEncoder,
{
    pub fn new(
        user_repo: &'a URepo,
        password_hasher: &'a PHasher,
        token_serv: TokenService<'a, TRepo, TEnc>,
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
        let mut err = Error::new("credentials", "invalid");

        let user = self
            .user_repo
            .find_by_username(&Username::new(username_or_email)?)
            .await
            .or(self
                .user_repo
                .find_by_email(&Email::new(username_or_email)?)
                .await)?;

        let user_password = match user.identity().password() {
            Some(password) => password.value(),
            None => return Err(err),
        };

        if self.password_hasher.compare(user_password, password) {
            let mut data = Data::new();
            data.add("user_id", user.base().id().value());
            let token = match self.token_serv.create(data).await {
                Ok(token) => token,
                Err(e) => return Err(err.wrap(e).build()),
            };

            return Ok((user, token));
        }
        Err(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::user::*;
    use crate::infrastructure::mocks::{self, *};
    use crate::infrastructure::persistence::inmem::*;

    #[tokio::test]
    async fn authenticate() -> Result<()> {
        let user_repo = InMemUserRepository::new();
        let password_hasher = FakePasswordHasher::new();
        let token_enc = FakeTokenEncoder::new();
        let token_repo = InMemTokenRepository::new();
        let token_serv = TokenService::new(&token_repo, &token_enc);

        let serv = AuthenticationService::new(&user_repo, &password_hasher, token_serv);

        let mut user = mocks::user1()?;
        user_repo.save(&mut user).await?;

        let (_, token) = serv.authenticate("username", "P@asswd!").await.unwrap();
        assert!(!token.token().is_empty());

        let (_, token) = serv
            .authenticate("username@email.com", "P@asswd!")
            .await
            .unwrap();
        assert!(!token.token().is_empty());

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

        Ok(())
    }
}
