use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::interaction::InteractionService;
use crate::domain::publication::{PublicationId, PublicationRepository};
use crate::domain::reader::{ReaderId, ReaderRepository};

pub struct DeleteReview<'a> {
    event_pub: &'a dyn EventPublisher,

    publication_repo: &'a dyn PublicationRepository,
    reader_repo: &'a dyn ReaderRepository,

    interaction_serv: &'a InteractionService,
}

impl<'a> DeleteReview<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        publication_repo: &'a dyn PublicationRepository,
        reader_repo: &'a dyn ReaderRepository,
        interaction_serv: &'a InteractionService,
    ) -> Self {
        DeleteReview {
            event_pub,
            publication_repo,
            reader_repo,
            interaction_serv,
        }
    }

    pub async fn exec(&self, auth_id: String, publication_id: String) -> Result<CommandResponse> {
        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let reader_id = ReaderId::new(auth_id)?;
        let reader = self.reader_repo.find_by_id(&reader_id).await?;

        self.interaction_serv
            .delete_review(&reader, &mut publication)
            .await?;

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
