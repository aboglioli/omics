use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::author::AuthorId;
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

    pub async fn exec(&self, auth_id: String, collection_id: String) -> Result<CommandResponse> {
        let collection_id = CollectionId::new(collection_id)?;
        let mut collection = self.collection_repo.find_by_id(&collection_id).await?;

        collection.delete(&AuthorId::new(auth_id)?)?;

        self.collection_repo.save(&mut collection).await?;

        self.event_pub
            .publish_all(collection.base().events()?)
            .await?;

        Ok(CommandResponse::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn valid() {
        let c = mocks::container();
        let uc = Delete::new(c.event_pub(), c.collection_repo());

        let author = mocks::author1();
        let mut collection = mocks::empty_collection1();
        c.collection_repo().save(&mut collection).await.unwrap();

        assert!(uc
            .exec(
                author.base().id().to_string(),
                collection.base().id().to_string()
            )
            .await
            .is_ok());

        let collection = c
            .collection_repo()
            .find_by_id(&collection.base().id())
            .await
            .unwrap();
        assert!(collection.base().deleted_at().is_some());
    }

    #[tokio::test]
    async fn invalid() {
        let c = mocks::container();
        let uc = Delete::new(c.event_pub(), c.collection_repo());

        let author = mocks::author1();
        let mut collection = mocks::empty_collection1();
        c.collection_repo().save(&mut collection).await.unwrap();

        assert!(uc
            .exec(
                author.base().id().to_string(),
                "#invalid-collection".to_owned()
            )
            .await
            .is_err());
        assert!(uc
            .exec(
                "#invald-author".to_owned(),
                collection.base().id().to_string()
            )
            .await
            .is_err());
    }
}
