use serde::Deserialize;

use common::result::Result;

use crate::domain::user::{UserId, UserService};

#[derive(Deserialize)]
pub struct ChangePasswordCommand {
    pub old_password: String,
    pub new_password: String,
}

impl ChangePasswordCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct ChangePassword<'a> {
    user_serv: &'a UserService,
}

impl<'a> ChangePassword<'a> {
    pub fn new(user_serv: &'a UserService) -> Self {
        ChangePassword { user_serv }
    }

    pub async fn exec(&self, user_id: String, cmd: ChangePasswordCommand) -> Result<()> {
        cmd.validate()?;

        let user_id = UserId::new(user_id)?;
        self.user_serv
            .change_password(&user_id, &cmd.old_password, &cmd.new_password)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn success() {
        let c = mocks::container();
        let uc = ChangePassword::new(c.user_serv());

        let mut user = mocks::user1();
        let old_password = user.identity().password().unwrap().to_string();
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc
            .exec(
                user.base().id().to_string(),
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
                user.base().id().to_string(),
                ChangePasswordCommand {
                    old_password: "invalid".to_owned(),
                    new_password: "new-password".to_owned(),
                }
            )
            .await
            .is_err());
    }
}
