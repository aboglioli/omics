use std::sync::Arc;

use async_trait::async_trait;

use common::event::{Event, EventHandler};
use common::result::Result;
use shared::event::PublicationEvent;

use crate::domain::author::AuthorRepository;
use crate::domain::publication::{
    PublicationId, PublicationRepository, Status as PublicationStatus,
};

pub struct PublicationCounterHandler {
    author_repo: Arc<dyn AuthorRepository>,
    publication_repo: Arc<dyn PublicationRepository>,
}

impl PublicationCounterHandler {
    pub fn new(
        author_repo: Arc<dyn AuthorRepository>,
        publication_repo: Arc<dyn PublicationRepository>,
    ) -> Self {
        PublicationCounterHandler {
            author_repo,
            publication_repo,
        }
    }
}

#[async_trait]
impl EventHandler for PublicationCounterHandler {
    fn topic(&self) -> &str {
        "publication"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event: PublicationEvent = serde_json::from_value(event.payload())?;

        match event {
            PublicationEvent::Published { id, .. }
            | PublicationEvent::ChangedToDraft { id, .. }
            | PublicationEvent::Deleted { id } => {
                let publication = self
                    .publication_repo
                    .find_by_id(&PublicationId::new(id)?)
                    .await?;

                let p_publications = self
                    .publication_repo
                    .search(
                        Some(publication.author_id()),
                        None,
                        None,
                        Some(&PublicationStatus::Published {
                            admin_id: None,
                            comment: None,
                        }),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .await?;

                let mut author = self.author_repo.find_by_id(publication.author_id()).await?;
                author.set_publications(p_publications.matching_criteria() as u32);
                self.author_repo.save(&mut author).await?;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
