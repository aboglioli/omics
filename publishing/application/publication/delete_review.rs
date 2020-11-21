use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::interaction::InteractionRepository;
use crate::domain::publication::{PublicationId, PublicationRepository};
use crate::domain::reader::ReaderRepository;

pub struct DeleteReview<'a> {
    event_pub: &'a dyn EventPublisher,

    interaction_repo: &'a dyn InteractionRepository,
    publication_repo: &'a dyn PublicationRepository,
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> DeleteReview<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        interaction_repo: &'a dyn InteractionRepository,
        publication_repo: &'a dyn PublicationRepository,
        reader_repo: &'a dyn ReaderRepository,
    ) -> Self {
        DeleteReview {
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
        if !auth_role.can("delete_review_from_publication") {
            return Err(Error::unauthorized());
        }

        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let reader = self.reader_repo.find_by_id(&auth_id).await?;

        let reviews = self
            .interaction_repo
            .find_reviews(Some(&auth_id), Some(&publication_id), None, None)
            .await?;

        if let Some(review) = reviews.first() {
            publication.delete_review(&reader, review.stars())?;
        }

        self.interaction_repo
            .delete_review(&auth_id, &publication_id)
            .await?;
        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
