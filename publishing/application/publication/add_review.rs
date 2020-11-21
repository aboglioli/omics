use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::interaction::{Comment, InteractionRepository, Stars};
use crate::domain::publication::{PublicationId, PublicationRepository};
use crate::domain::reader::ReaderRepository;

#[derive(Deserialize)]
pub struct AddReviewCommand {
    pub stars: u8,
    pub comment: String,
}

pub struct AddReview<'a> {
    event_pub: &'a dyn EventPublisher,

    interaction_repo: &'a dyn InteractionRepository,
    publication_repo: &'a dyn PublicationRepository,
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> AddReview<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        interaction_repo: &'a dyn InteractionRepository,
        publication_repo: &'a dyn PublicationRepository,
        reader_repo: &'a dyn ReaderRepository,
    ) -> Self {
        AddReview {
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
        cmd: AddReviewCommand,
    ) -> Result<CommandResponse> {
        if !auth_role.can("add_review_to_publication") {
            return Err(Error::unauthorized());
        }

        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let reader = self.reader_repo.find_by_id(&auth_id).await?;

        let stars = Stars::new(cmd.stars)?;
        let comment = Comment::new(cmd.comment)?;

        let mut review = publication.review(&reader, stars, comment)?;

        self.interaction_repo.save_review(&mut review).await?;
        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
