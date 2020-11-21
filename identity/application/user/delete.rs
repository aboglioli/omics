use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::user::{UserId, UserRepository};
use crate::UserIdAndRole;

pub struct Delete<'a> {
    event_pub: &'a dyn EventPublisher,

    user_repo: &'a dyn UserRepository,
}

impl<'a> Delete<'a> {
    pub fn new(event_pub: &'a dyn EventPublisher, user_repo: &'a dyn UserRepository) -> Self {
        Delete {
            event_pub,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        user_id: String,
    ) -> Result<CommandResponse> {
        let user_id = UserId::new(user_id)?;
        if !auth_role.can("delete_all_users") {
            if auth_id != user_id || !auth_role.can("delete_own_user") {
                return Err(Error::unauthorized());
            }
        }

        let mut user = self.user_repo.find_by_id(&user_id).await?;

        user.delete()?;

        self.user_repo.delete(user.base().id()).await?;

        self.event_pub.publish_all(user.events().to_vec()?).await?;

        Ok(CommandResponse::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn delete() {
        let c = mocks::container();
        let uc = Delete::new(c.event_pub(), c.user_repo());

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
        let role = mocks::role("User");

        let user_id = user.base().id().to_string();
        assert!(uc
            .exec((user.base().id().clone(), role.clone()), user_id.clone())
            .await
            .is_ok());
        assert!(uc
            .exec((user.base().id().clone(), role.clone()), user_id)
            .await
            .is_err());

        assert_eq!(c.event_pub().events().await.len(), 1);
    }
}
