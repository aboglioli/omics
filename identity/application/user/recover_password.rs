use serde::Deserialize;
use uuid::Uuid;

use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::role::RoleRepository;
use crate::domain::user::{Email, Password, UserRepository, UserService};

#[derive(Deserialize)]
pub struct RecoverPasswordCommand {
    pub email: String,
}

pub struct RecoverPassword<'a> {
    event_pub: &'a dyn EventPublisher,

    role_repo: &'a dyn RoleRepository,
    user_repo: &'a dyn UserRepository,

    user_serv: &'a UserService,
}

impl<'a> RecoverPassword<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        role_repo: &'a dyn RoleRepository,
        user_repo: &'a dyn UserRepository,
        user_serv: &'a UserService,
    ) -> Self {
        RecoverPassword {
            event_pub,
            role_repo,
            user_repo,
            user_serv,
        }
    }

    pub async fn exec(&self, cmd: RecoverPasswordCommand) -> Result<CommandResponse> {
        let email = Email::new(cmd.email)?;
        let mut user = self.user_repo.find_by_email(&email).await?;

        let role = self.role_repo.find_by_user_id(user.base().id()).await?;
        if !role.can("recover_user_password") {
            return Err(Error::unauthorized());
        }

        let tmp_password = Uuid::new_v4().to_string();
        let hashed_password = self.user_serv.generate_password(&tmp_password)?;
        let password = Password::new(hashed_password)?;

        user.recover_password(password, tmp_password)?;

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
    async fn non_existing_user() {
        let c = mocks::container();
        let uc = RecoverPassword::new(c.event_pub(), c.role_repo(), c.user_repo(), c.user_serv());

        assert!(uc
            .exec(RecoverPasswordCommand {
                email: "non-existing@omics.com".to_owned(),
            })
            .await
            .is_err())
    }

    #[tokio::test]
    async fn password_recovery_code_generated() {
        let c = mocks::container();
        let uc = RecoverPassword::new(c.event_pub(), c.role_repo(), c.user_repo(), c.user_serv());

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
            .exec(RecoverPasswordCommand {
                email: user.identity().email().to_string(),
            })
            .await
            .is_ok());

        let user = c.user_repo().find_by_id(&user.base().id()).await.unwrap();
        assert_ne!(user.identity().password().unwrap().value(), old_password);

        assert!(!c.event_pub().events().await.is_empty());
    }
}
