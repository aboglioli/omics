use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::collection::{CollectionId, CollectionRepository};
use crate::domain::interaction::InteractionRepository;
use crate::domain::reader::{ReaderId, ReaderRepository};

pub struct RemoveFromFavorites<'a> {
    event_pub: &'a dyn EventPublisher,

    collection_repo: &'a dyn CollectionRepository,
    interaction_repo: &'a dyn InteractionRepository,
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> RemoveFromFavorites<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        collection_repo: &'a dyn CollectionRepository,
        interaction_repo: &'a dyn InteractionRepository,
        reader_repo: &'a dyn ReaderRepository,
    ) -> Self {
        RemoveFromFavorites {
            event_pub,
            collection_repo,
            interaction_repo,
            reader_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        collection_id: String,
    ) -> Result<CommandResponse> {
        if !auth_role.can("remove_collection_from_favorites") {
            return Err(Error::unauthorized());
        }

        let mut reader = self.reader_repo.find_by_id(&auth_id).await?;

        let collection_id = CollectionId::new(collection_id)?;
        let collection = self.collection_repo.find_by_id(&collection_id).await?;

        self.interaction_repo
            .delete_collection_favorite(&auth_id, &collection_id)
            .await?;

        reader.remove_collection_from_favorites(&collection)?;

        self.event_pub
            .publish_all(reader.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
