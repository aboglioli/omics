use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::interaction::InteractionRepository;
use crate::domain::publication::{PublicationId, PublicationRepository};
use crate::domain::reader::ReaderRepository;

pub struct RemoveFromFavorites<'a> {
    event_pub: &'a dyn EventPublisher,

    interaction_repo: &'a dyn InteractionRepository,
    publication_repo: &'a dyn PublicationRepository,
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> RemoveFromFavorites<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        interaction_repo: &'a dyn InteractionRepository,
        publication_repo: &'a dyn PublicationRepository,
        reader_repo: &'a dyn ReaderRepository,
    ) -> Self {
        RemoveFromFavorites {
            event_pub,
            interaction_repo,
            publication_repo,
            reader_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        publication_id: String,
    ) -> Result<CommandResponse> {
        if !auth_role.can("add_remove_publication_from_favorites") {
            return Err(Error::unauthorized());
        }

        let mut reader = self.reader_repo.find_by_id(&auth_id).await?;

        let publication_id = PublicationId::new(publication_id)?;
        let publication = self.publication_repo.find_by_id(&publication_id).await?;

        self.interaction_repo
            .delete_publication_favorite(&auth_id, &publication_id)
            .await?;

        reader.remove_publication_from_favorites(&publication)?;

        self.event_pub
            .publish_all(reader.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
