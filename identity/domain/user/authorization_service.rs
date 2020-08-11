use common::error::Error;
use common::result::Result;

use crate::domain::token::{Token, TokenEncoder, TokenRepository, TokenService};
use crate::domain::user::{User, UserId, UserRepository};

pub struct AuthorizationService<'a, URepo, TRepo, TEnc> {
    user_repo: &'a URepo,

    token_serv: TokenService<'a, TRepo, TEnc>,
}

impl<'a, URepo, TRepo, TEnc> AuthorizationService<'a, URepo, TRepo, TEnc>
where
    URepo: UserRepository,
    TRepo: TokenRepository,
    TEnc: TokenEncoder,
{
    pub fn new(user_repo: &'a URepo, token_serv: TokenService<'a, TRepo, TEnc>) -> Self {
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
        Err(Error::new("authorization", "unauthorized"))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn authenticate() {
        // TODO: implement
    }
}
