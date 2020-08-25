use common::event::EventPublisher;
use common::result::Result;

use crate::application::dtos::CommandResponse;
use crate::domain::collection::{CollectionId, CollectionRepository};
use crate::domain::publication::PublicationId;

pub struct RemovePublication<'a> {
    event_pub: &'a dyn EventPublisher,

    collection_repo: &'a dyn CollectionRepository,
}

impl<'a> RemovePublication<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        collection_repo: &'a dyn CollectionRepository,
    ) -> Self {
        RemovePublication {
            event_pub,
            collection_repo,
        }
    }

    pub async fn exec(
        &self,
        collection_id: String,
        publication_id: String,
    ) -> Result<CommandResponse> {
        let collection_id = CollectionId::new(collection_id)?;
        let mut collection = self.collection_repo.find_by_id(&collection_id).await?;

        let publication_id = PublicationId::new(publication_id)?;

        collection.remove_item(&publication_id)?;

        self.collection_repo.save(&mut collection).await?;

        self.event_pub
            .publish_all(collection.base().events()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
