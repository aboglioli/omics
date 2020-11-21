use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::collection::{CollectionId, CollectionRepository};

pub struct Delete<'a> {
    event_pub: &'a dyn EventPublisher,

    collection_repo: &'a dyn CollectionRepository,
}

impl<'a> Delete<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        collection_repo: &'a dyn CollectionRepository,
    ) -> Self {
        Delete {
            event_pub,
            collection_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        collection_id: String,
    ) -> Result<CommandResponse> {
        if !auth_role.can("delete_collection") {
            return Err(Error::unauthorized());
        }

        let collection_id = CollectionId::new(collection_id)?;
        let mut collection = self.collection_repo.find_by_id(&collection_id).await?;

        if collection.author_id() != &auth_id {
            return Err(Error::not_owner("collection"));
        }

        collection.delete()?;

        self.collection_repo.delete(collection.base().id()).await?;

        self.event_pub
            .publish_all(collection.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use identity::domain::user::UserId;
    use identity::mocks as identity_mocks;

    use crate::mocks;

    #[tokio::test]
    async fn valid() {
        let c = mocks::container();
        let uc = Delete::new(c.event_pub(), c.collection_repo());

        let mut collection = mocks::collection(
            "#collection01",
            "#user01",
            "User",
            "category-1",
            vec!["Tag 1", "Tag 2"],
            "cover.jpg",
        );
        c.collection_repo().save(&mut collection).await.unwrap();
        let role = identity_mocks::role("User");

        assert!(uc
            .exec(
                (UserId::new("#user01").unwrap(), role),
                collection.base().id().to_string()
            )
            .await
            .is_ok());

        assert!(c
            .collection_repo()
            .find_by_id(collection.base().id())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn invalid() {
        let c = mocks::container();
        let uc = Delete::new(c.event_pub(), c.collection_repo());
        let role = identity_mocks::role("User");

        let mut collection = mocks::collection(
            "#collection01",
            "#user01",
            "User",
            "category-1",
            vec!["Tag 1", "Tag 2"],
            "cover.jpg",
        );
        c.collection_repo().save(&mut collection).await.unwrap();

        assert!(uc
            .exec(
                (UserId::new("#user01").unwrap(), role.clone()),
                "#invalid-collection".to_owned()
            )
            .await
            .is_err());
        assert!(uc
            .exec(
                (UserId::new("#invald-author").unwrap(), role),
                collection.base().id().to_string()
            )
            .await
            .is_err());
    }
}
