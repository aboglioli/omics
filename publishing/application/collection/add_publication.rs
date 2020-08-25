use common::event::EventPublisher;
use common::result::Result;

use crate::application::dtos::CommandResponse;
use crate::domain::collection::{CollectionId, CollectionRepository};
use crate::domain::publication::{PublicationId, PublicationRepository};

pub struct AddPublication<'a> {
    event_pub: &'a dyn EventPublisher,

    collection_repo: &'a dyn CollectionRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> AddPublication<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        collection_repo: &'a dyn CollectionRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        AddPublication {
            event_pub,
            collection_repo,
            publication_repo,
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
        let publication = self.publication_repo.find_by_id(&publication_id).await?;

        collection.add_item(&publication)?;

        self.collection_repo.save(&mut collection).await?;

        self.event_pub
            .publish_all(collection.base().events()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
