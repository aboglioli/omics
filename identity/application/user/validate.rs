use common::event::EventPublisher;
use common::result::Result;

use crate::domain::user::{UserId, UserRepository, Validation};

pub struct Validate<'a, URepo, EPub> {
    event_pub: &'a EPub,

    user_repo: &'a URepo,
}

impl<'a, URepo, EPub> Validate<'a, URepo, EPub>
where
    EPub: EventPublisher,
    URepo: UserRepository,
{
    pub fn new(event_pub: &'a EPub, user_repo: &'a URepo) -> Self {
        Validate {
            event_pub,
            user_repo,
        }
    }

    pub async fn exec(&self, user_id: &UserId, validation: &Validation) -> Result<()> {
        let mut user = self.user_repo.find_by_id(user_id).await?;

        user.validate(validation)?;

        self.user_repo.save(&mut user).await?;

        self.event_pub.publish_all(user.base().events()?).await?;

        Ok(())
    }
}
