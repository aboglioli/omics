use uuid::Uuid;

use common::result::Result;

use crate::domain::token::{TokenEncoder, TokenRepository};
use crate::domain::user::{AuthService, Password, PasswordHasher, UserId, UserRepository};

pub struct RecoverPassword<'a, URepo, PHasher, TRepo, TEnc> {
    user_repo: &'a URepo,
    auth_serv: AuthService<'a, URepo, PHasher, TRepo, TEnc>,
}

impl<'a, URepo, PHasher, TRepo, TEnc> RecoverPassword<'a, URepo, PHasher, TRepo, TEnc>
where
    URepo: UserRepository,
    PHasher: PasswordHasher,
    TRepo: TokenRepository,
    TEnc: TokenEncoder,
{
    pub fn new(
        user_repo: &'a URepo,
        auth_serv: AuthService<'a, URepo, PHasher, TRepo, TEnc>,
    ) -> Self {
        RecoverPassword {
            user_repo,
            auth_serv,
        }
    }

    pub async fn exec(&self, user_id: &UserId) -> Result<()> {
        let mut user = self.user_repo.find_by_id(user_id).await?;

        let tmp_password = Uuid::new_v4();
        let hashed_password = self
            .auth_serv
            .generate_password(&tmp_password.to_string())?;
        let password = Password::new(&hashed_password)?;

        user.set_password(password)?;

        Ok(())
    }
}
