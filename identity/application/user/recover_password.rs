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
        user_repo: &'a URepo,
        user_serv: UserService<'a, URepo, PHasher>,
        event_pub: &'a EPub,
    ) -> Self {
        RecoverPassword {
            user_repo,
            user_serv,
            event_pub,
        }
    }

    pub async fn exec(&self, user_id: &UserId) -> Result<()> {
        let mut user = self.user_repo.find_by_id(user_id).await?;

        let tmp_password = Uuid::new_v4().to_string();
        let hashed_password = self.user_serv.generate_password(&tmp_password)?;
        let password = Password::new(&hashed_password)?;

        user.recover_password(password, &tmp_password)?;

        self.event_pub.publish_all(user.base().events()?).await?;

        Ok(())
    }
}
