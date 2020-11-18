use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;
use common::error::Error;

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
        (auth_id, auth_role): UserIdAndRole,
        collection_id: String,
        publication_id: String,
    ) -> Result<CommandResponse> {
        if !auth_role.can("remove_publication_from_collection") {
            return Err(Error::unauthorized());
        }

        let collection_id = CollectionId::new(collection_id)?;
        let mut collection = self.collection_repo.find_by_id(&collection_id).await?;

        if collection.author_id().value() != &auth_id {
            return Err(Error::not_owner("collection"));
        }

        collection.remove_item(&PublicationId::new(publication_id)?)?;

        self.collection_repo.save(&mut collection).await?;

        self.event_pub
            .publish_all(collection.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
