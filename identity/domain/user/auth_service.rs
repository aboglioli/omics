use common::error::Error;

use crate::domain::token::{Data, Token, TokenEncoder, TokenRepository, TokenService};
use crate::domain::user::{
    Email, Password, PasswordHasher, User, UserId, UserRepository, Username,
};

pub struct AuthService<'a, URepo, PHasher, TRepo, TEnc> {
    user_repo: &'a URepo,
    password_hasher: &'a PHasher,
    token_serv: TokenService<'a, TRepo, TEnc>,
}

impl<'a, URepo, PHasher, TRepo, TEnc> AuthService<'a, URepo, PHasher, TRepo, TEnc>
where
    URepo: UserRepository,
    PHasher: PasswordHasher,
    TRepo: TokenRepository,
    TEnc: TokenEncoder,
{
    pub fn new(
        user_repo: &'a URepo,
        token_serv: TokenService<'a, TRepo, TEnc>,
        password_hasher: &'a PHasher,
    ) -> Self {
        AuthService {
            user_repo,
            token_serv,
            password_hasher,
        }
    }

    pub async fn authenticate(
        &self,
        username_or_email: &str,
        password: &str,
    ) -> Result<Token, Error> {
        let mut err = Error::pair("credentials", "invalid");

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
            data.add("user_id", &user.base().id());
            let token = match self.token_serv.create(data).await {
                Ok(token) => token,
                Err(e) => return Err(err.wrap(e).build()),
            };

            return Ok(token);
        }
        Err(Error::application().set_code("invalid_credentials").build())
    }

    pub async fn authorize(&self, token: &Token) -> Result<User, Error> {
        let data = self.token_serv.validate(token).await?;
        if let Some(user_id) = data.get("user_id") {
            let user = self.user_repo.find_by_id(user_id).await?;
            return Ok(user);
        }
        Err(Error::application())
    }

    pub async fn available(&self, username: &str, email: &str) -> Result<bool, Error> {
        let mut err = Error::application();
        if self
            .user_repo
            .find_by_username(&Username::new(username)?)
            .await
            .is_ok()
        {
            err.add_context("username", "not_available");
        }
        if self
            .user_repo
            .find_by_email(&Email::new(email)?)
            .await
            .is_ok()
        {
            err.add_context("email", "not_available");
        }

        if err.has_context() {
            return Err(err);
        }

        Ok(true)
    }

    pub async fn change_password(
        &self,
        user_id: &UserId,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), Error> {
        let mut user = self.user_repo.find_by_id(user_id).await?;

        let user_password = match user.identity().password() {
            Some(password) => password.value(),
            None => return Err(Error::pair("password", "unavailable")),
        };

        if !self.password_hasher.compare(user_password, old_password) {
            return Err(Error::application().set_code("invalid_password").build());
        }

        let hashed_password = self.password_hasher.hash(new_password)?;

        let password = Password::new(&hashed_password)?;
        user.set_password(password)?;

        self.user_repo.save(&mut user).await?;

        Ok(())
    }

    pub fn generate_password(&self, plain_pasword: &str) -> Result<String, Error> {
        self.password_hasher.hash(plain_pasword)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//
//     use crate::domain::user::*;
//     use crate::infrastructure::mocks::{self, *};
//     use crate::infrastructure::persistence::inmem::*;
//
//     #[test]
//     fn authenticate() -> Result<(), Error> {
//         let user_repo = Arc::new(InMemUserRepository::new());
//         let password_hasher = Arc::new(FakePasswordHasher::new());
//         let token_enc = Arc::new(FakeTokenEncoder::new());
//         let token_repo = Arc::new(InMemTokenRepository::new());
//         let token_serv = Arc::new(TokenService::new(token_enc.clone(), token_repo.clone()));
//
//         let serv = AuthService::new(
//             user_repo.clone(),
//             token_serv.clone(),
//             password_hasher.clone(),
//         );
//
//         let mut user = mocks::user1()?;
//         user_repo.save(&mut user)?;
//
//         let res = serv.authenticate("username", "P@asswd!");
//         assert!(res.is_ok());
//         assert!(!res.unwrap().token().is_empty());
//
//         let res = serv.authenticate("username@email.com", "P@asswd!");
//         assert!(res.is_ok());
//         assert!(!res.unwrap().token().is_empty());
//
//         assert!(serv.authenticate("user2", "user123").is_err());
//         assert!(serv.authenticate("user1", "user124").is_err());
//         assert!(serv.authenticate("user@email.com.ar", "user123").is_err());
//         assert!(serv.authenticate("user@email.com", "user124").is_err());
//
//         Ok(())
//     }
// }
