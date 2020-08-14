
use common::event::EventPublisher;
use common::result::Result;

use crate::domain::interaction::{InteractionRepository, InteractionService};
use crate::domain::publication::{PublicationId, PublicationRepository};
use crate::domain::reader::{ReaderId, ReaderRepository};

pub struct Unlike<'a, EPub, PRepo, RRepo, IRepo> {
    event_pub: &'a EPub,

    publication_repo: &'a PRepo,
    reader_repo: &'a RRepo,

    interaction_serv: InteractionService<'a, IRepo>,
}

impl<'a, EPub, PRepo, RRepo, IRepo> Unlike<'a, EPub, PRepo, RRepo, IRepo>
where
    EPub: EventPublisher,
    PRepo: PublicationRepository,
    RRepo: ReaderRepository,
    IRepo: InteractionRepository,
{
    pub fn new(
        event_pub: &'a EPub,
        publication_repo: &'a PRepo,
        reader_repo: &'a RRepo,
        interaction_serv: InteractionService<'a, IRepo>,
    ) -> Self {
        Unlike {
            event_pub,
            publication_repo,
            reader_repo,
            interaction_serv,
        }
    }

    pub async fn exec(&self, reader_id: String, publication_id: String) -> Result<()> {
        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let reader_id = ReaderId::new(reader_id)?;
        let reader = self.reader_repo.find_by_id(&reader_id).await?;

        self.interaction_serv
            .delete_like(&reader, &mut publication)
            .await?;

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.base().events()?)
            .await?;

        Ok(())
    }
}
