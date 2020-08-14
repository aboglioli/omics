use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;

use crate::domain::collection::{CollectionId, CollectionRepository};

pub struct Delete<'a, EPub, CRepo> {
    event_pub: &'a EPub,

    collection_repo: &'a CRepo,
}

impl<'a, EPub, CRepo> Delete<'a, EPub, CRepo>
where
    EPub: EventPublisher,
    CRepo: CollectionRepository,
{
    pub fn new(event_pub: &'a EPub, collection_repo: &'a CRepo) -> Self {
        Delete {
            event_pub,
            collection_repo,
        }
    }

    pub async fn exec(&self, author_id: String, publication_id: String) -> Result<()> {
        let publication_id = CollectionId::new(publication_id)?;
        let mut collection = self.collection_repo.find_by_id(&publication_id).await?;

        if collection.author_id().value() != author_id {
            return Err(Error::new("collection", "unauthorized"));
        }

        collection.delete()?;

        self.collection_repo.save(&mut collection).await?;

        self.event_pub
            .publish_all(collection.base().events()?)
            .await?;

        Ok(())
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
                author.base().id().value().to_owned(),
                collection.base().id().value().to_owned()
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
                author.base().id().value().to_owned(),
                "#invalid-collection".to_owned()
            )
            .await
            .is_err());
        assert!(uc
            .exec(
                "#invald-author".to_owned(),
                collection.base().id().value().to_owned()
            )
            .await
            .is_err());
    }
}
