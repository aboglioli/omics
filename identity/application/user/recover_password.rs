use uuid::Uuid;

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::user::{Password, PasswordHasher, UserId, UserRepository, UserService};

pub struct RecoverPassword<'a, EPub, URepo, PHasher> {
    event_pub: &'a EPub,

    user_repo: &'a URepo,

    user_serv: UserService<'a, URepo, PHasher>,
}

impl<'a, EPub, URepo, PHasher> RecoverPassword<'a, EPub, URepo, PHasher>
where
    EPub: EventPublisher,
    URepo: UserRepository,
    PHasher: PasswordHasher,
{
    pub fn new(
        event_pub: &'a EPub,
        user_repo: &'a URepo,
        user_serv: UserService<'a, URepo, PHasher>,
    ) -> Self {
        RecoverPassword {
            user_repo,
            user_serv,
            event_pub,
        }
    }

    pub async fn exec(&self, user_id: String) -> Result<()> {
        let user_id = UserId::new(user_id)?;
        let mut user = self.user_repo.find_by_id(&user_id).await?;

        let tmp_password = Uuid::new_v4().to_string();
        let hashed_password = self.user_serv.generate_password(&tmp_password)?;
        let password = Password::new(hashed_password)?;

        user.recover_password(password, &tmp_password)?;

        self.user_repo.save(&mut user).await?;

        self.event_pub.publish_all(user.base().events()?).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn non_existing_user() {
        let c = mocks::container();
        let uc = RecoverPassword::new(c.event_pub(), c.user_repo(), c.user_serv());

        let user = mocks::user1();
        assert!(uc.exec(user.base().id().value().to_owned()).await.is_err())
    }

    #[tokio::test]
    async fn password_recovery_code_generated() {
        let c = mocks::container();
        let uc = RecoverPassword::new(c.event_pub(), c.user_repo(), c.user_serv());

        let mut user = mocks::user1();
        let old_password = user.identity().password().unwrap().value().to_owned();
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc.exec(user.base().id().value().to_owned()).await.is_ok());

        let user = c.user_repo().find_by_id(&user.base().id()).await.unwrap();
        assert_ne!(user.identity().password().unwrap().value(), old_password);

        assert_eq!(c.event_pub().events().await.len(), 1);
    }
}
