use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;

use crate::domain::publication::{PublicationId, PublicationRepository};

pub struct Delete<'a, EPub, PRepo> {
    event_pub: &'a EPub,

    publication_repo: &'a PRepo,
}

impl<'a, EPub, PRepo> Delete<'a, EPub, PRepo>
where
    EPub: EventPublisher,
    PRepo: PublicationRepository,
{
    pub fn new(event_pub: &'a EPub, publication_repo: &'a PRepo) -> Self {
        Delete {
            event_pub,
            publication_repo,
        }
    }

    async fn exec(&self, author_id: String, publication_id: String) -> Result<()> {
        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        if publication.author_id().value() != author_id {
            return Err(Error::new("publication", "unauthorized"));
        }

        publication.delete()?;

        self.event_pub
            .publish_all(publication.base().events()?)
            .await?;

        Ok(())
    }
}
