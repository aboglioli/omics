use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::role::RoleRepository;
use crate::domain::user::{UserId, UserRepository, UserService};

#[derive(Deserialize)]
pub struct ChangePasswordCommand {
    pub old_password: String,
    pub new_password: String,
}

pub struct ChangePassword<'a> {
    event_pub: &'a dyn EventPublisher,

    role_repo: &'a dyn RoleRepository,
    user_repo: &'a dyn UserRepository,

    user_serv: &'a UserService,
}

impl<'a> ChangePassword<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        role_repo: &'a dyn RoleRepository,
        user_repo: &'a dyn UserRepository,
        user_serv: &'a UserService,
    ) -> Self {
        ChangePassword {
            event_pub,
            role_repo,
            user_repo,
            user_serv,
        }
    }

    pub async fn exec(
        &self,
        user_id: String,
        cmd: ChangePasswordCommand,
    ) -> Result<CommandResponse> {
        let user_id = UserId::new(user_id)?;
        let role = self.role_repo.find_by_user_id(&user_id).await?;
        if !role.can("change_user_password") {
            return Err(Error::unauthorized());
        }

        let mut user = self.user_repo.find_by_id(&user_id).await?;

        self.user_serv
            .change_password(&mut user, &cmd.old_password, &cmd.new_password)
            .await?;

        self.user_repo.save(&mut user).await?;

        self.event_pub.publish_all(user.events().to_vec()?).await?;

        Ok(CommandResponse::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn success() {
        let c = mocks::container();
        let uc = ChangePassword::new(c.event_pub(), c.role_repo(), c.user_repo(), c.user_serv());

        let mut user = mocks::user(
            "user-1",
            "username",
            "user@omics.com",
            "P@asswd!",
            true,
            None,
            None,
            "user",
        );
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
        let uc = ChangePassword::new(c.event_pub(), c.role_repo(), c.user_repo(), c.user_serv());

        let mut user = mocks::user(
            "user-1",
            "username",
            "user@omics.com",
            "P@asswd!",
            true,
            None,
            None,
            "user",
        );
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
