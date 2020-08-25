use std::sync::Arc;

use async_trait::async_trait;

use common::error::Error;
use common::event::{Event, EventHandler};
use common::result::Result;
use shared::event::PublicationEvent;

use crate::domain::interaction::{InteractionRepository, Stars};
use crate::domain::publication::{PublicationId, PublicationRepository};
use crate::domain::reader::ReaderId;

// TODO: not used
pub struct InteractionHandler {
    interaction_repo: Arc<dyn InteractionRepository>,
    publication_repo: Arc<dyn PublicationRepository>,
}

impl InteractionHandler {
    pub fn new(
        interaction_repo: Arc<dyn InteractionRepository>,
        publication_repo: Arc<dyn PublicationRepository>,
    ) -> Self {
        InteractionHandler {
            interaction_repo,
            publication_repo,
        }
    }
}

#[async_trait]
impl EventHandler for InteractionHandler {
    fn topic(&self) -> &str {
        "publication"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event: PublicationEvent = serde_json::from_slice(event.payload())
            .map_err(|err| Error::new("interaction_handler", "deserialize").wrap_raw(err))?;

        match event {
            PublicationEvent::Viewed {
                reader_id: _,
                publication_id,
                unique,
            } => {
                let mut publication = self
                    .publication_repo
                    .find_by_id(&PublicationId::new(publication_id)?)
                    .await?;

                publication.statistics_mut().add_view(unique);

                self.publication_repo.save(&mut publication).await?;

                return Ok(true);
            }
            PublicationEvent::Read {
                reader_id: _,
                publication_id,
            } => {
                let mut publication = self
                    .publication_repo
                    .find_by_id(&PublicationId::new(publication_id)?)
                    .await?;

                publication.statistics_mut().add_reading();

                self.publication_repo.save(&mut publication).await?;

                return Ok(true);
            }
            PublicationEvent::Liked {
                reader_id: _,
                publication_id,
            } => {
                let mut publication = self
                    .publication_repo
                    .find_by_id(&PublicationId::new(publication_id)?)
                    .await?;

                publication.statistics_mut().add_like();

                self.publication_repo.save(&mut publication).await?;

                return Ok(true);
            }
            PublicationEvent::Unliked {
                reader_id: _,
                publication_id,
            } => {
                let mut publication = self
                    .publication_repo
                    .find_by_id(&PublicationId::new(publication_id)?)
                    .await?;

                publication.statistics_mut().remove_like();

                self.publication_repo.save(&mut publication).await?;

                return Ok(true);
            }
            PublicationEvent::Reviewed {
                reader_id: _,
                publication_id,
                stars,
                comment: _,
            } => {
                let mut publication = self
                    .publication_repo
                    .find_by_id(&PublicationId::new(publication_id)?)
                    .await?;

                publication.statistics_mut().add_review(&Stars::new(stars)?);

                self.publication_repo.save(&mut publication).await?;

                return Ok(true);
            }
            PublicationEvent::ReviewDeleted {
                reader_id,
                publication_id,
            } => {
                let publication_id = PublicationId::new(publication_id)?;
                let reviews = self
                    .interaction_repo
                    .find_reviews(
                        Some(&ReaderId::new(reader_id)?),
                        Some(&publication_id),
                        None,
                        None,
                    )
                    .await?;

                if reviews.len() == 1 {
                    let review = &reviews[0];
                    let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

                    publication.statistics_mut().remove_review(review.stars());

                    self.publication_repo.save(&mut publication).await?;

                    return Ok(true);
                }
            }
            _ => {}
        }

        Ok(false)
    }
}
