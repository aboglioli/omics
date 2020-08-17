use common::event::EventPublisher;
use common::result::Result;

use crate::domain::collection::{CollectionId, CollectionRepository};
use crate::domain::publication::PublicationId;

pub struct RemovePublication<'a, EPub, CRepo> {
    event_pub: &'a EPub,

    collection_repo: &'a CRepo,
}

impl<'a, EPub, CRepo> RemovePublication<'a, EPub, CRepo>
where
    EPub: EventPublisher,
    CRepo: CollectionRepository,
{
    pub fn new(event_pub: &'a EPub, collection_repo: &'a CRepo) -> Self {
        RemovePublication {
            event_pub,
            collection_repo,
        }
    }

    pub async fn exec(&self, collection_id: String, publication_id: String) -> Result<()> {
        let collection_id = CollectionId::new(collection_id)?;
        let mut collection = self.collection_repo.find_by_id(&collection_id).await?;

        let publication_id = PublicationId::new(publication_id)?;

        collection.remove_item(&publication_id)?;

        self.collection_repo.save(&mut collection).await?;

        self.event_pub
            .publish_all(collection.base().events()?)
            .await?;

        Ok(())
    }
}
