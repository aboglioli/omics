use serde::Deserialize;

use common::result::Result;

use crate::application::util;
use crate::domain::user::{PasswordHasher, User, UserId, UserRepository, UserService};

#[derive(Deserialize)]
pub struct ChangePasswordCommand {
    old_password: String,
    new_password: String,
}

impl ChangePasswordCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct ChangePassword<'a, URepo, PHasher> {
    user_serv: UserService<'a, URepo, PHasher>,
}

impl<'a, URepo, PHasher> ChangePassword<'a, URepo, PHasher>
where
    URepo: UserRepository,
    PHasher: PasswordHasher,
{
    pub fn new(user_serv: UserService<'a, URepo, PHasher>) -> Self {
        ChangePassword { user_serv }
    }

    pub async fn exec(
        &self,
        auth_user: &User,
        user_id: &UserId,
        cmd: ChangePasswordCommand,
    ) -> Result<()> {
        util::is_authorized(auth_user, user_id)?;

        cmd.validate()?;

        self.user_serv
            .change_password(user_id, &cmd.old_password, &cmd.new_password)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn success_owner_valid_password() {
        let c = mocks::container();
        let uc = ChangePassword::new(c.user_serv());

        let mut user = mocks::user1();
        let old_password = user.identity().password().unwrap().value().to_owned();
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc
            .exec(
                &user,
                &user.base().id(),
                ChangePasswordCommand {
                    old_password: "P@asswd!".to_owned(),
                    new_password: "new-password".to_owned(),
                }
            )
            .await
            .is_ok());

        let user = c.user_repo().find_by_id(&user.base().id()).await.unwrap();
        assert_ne!(user.identity().password().unwrap().value(), old_password);
    }

    #[tokio::test]
    async fn invalid_password() {
        let c = mocks::container();
        let uc = ChangePassword::new(c.user_serv());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc
            .exec(
                &user,
                &user.base().id(),
                ChangePasswordCommand {
                    old_password: "invalid".to_owned(),
                    new_password: "new-password".to_owned(),
                }
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn not_owner() {
        let c = mocks::container();
        let uc = ChangePassword::new(c.user_serv());

        let mut user1 = mocks::user1();
        c.user_repo().save(&mut user1).await.unwrap();

        let mut user2 = mocks::user2();
        c.user_repo().save(&mut user2).await.unwrap();

        assert!(uc
            .exec(
                &user1,
                &user2.base().id(),
                ChangePasswordCommand {
                    old_password: "P@asswd!".to_owned(),
                    new_password: "new-password".to_owned(),
                }
            )
            .await
            .is_err());
    }
}
