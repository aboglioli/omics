use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::collection::{CollectionId, CollectionRepository};
use crate::domain::interaction::InteractionRepository;
use crate::domain::reader::ReaderRepository;

pub struct AddToFavorites<'a> {
    event_pub: &'a dyn EventPublisher,

    collection_repo: &'a dyn CollectionRepository,
    interaction_repo: &'a dyn InteractionRepository,
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> AddToFavorites<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        collection_repo: &'a dyn CollectionRepository,
        interaction_repo: &'a dyn InteractionRepository,
        reader_repo: &'a dyn ReaderRepository,
    ) -> Self {
        AddToFavorites {
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
        if !auth_role.can("add_remove_collection_from_favorites") {
            return Err(Error::unauthorized());
        }

        let collection_id = CollectionId::new(collection_id)?;
        let collection = self.collection_repo.find_by_id(&collection_id).await?;

        let mut reader = self.reader_repo.find_by_id(&auth_id).await?;

        let mut favorite = reader.add_collection_to_favorites(&collection)?;

        self.interaction_repo
            .save_collection_favorite(&mut favorite)
            .await?;

        self.event_pub
            .publish_all(reader.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
