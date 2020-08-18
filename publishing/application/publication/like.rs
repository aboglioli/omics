use common::event::EventPublisher;
use common::result::Result;

use crate::domain::interaction::InteractionService;
use crate::domain::publication::{PublicationId, PublicationRepository};
use crate::domain::reader::{ReaderId, ReaderRepository};

pub struct Like<'a> {
    event_pub: &'a dyn EventPublisher,

    publication_repo: &'a dyn PublicationRepository,
    reader_repo: &'a dyn ReaderRepository,

    interaction_serv: &'a InteractionService,
}

impl<'a> Like<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        publication_repo: &'a dyn PublicationRepository,
        reader_repo: &'a dyn ReaderRepository,
        interaction_serv: &'a InteractionService,
    ) -> Self {
        Like {
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
            .add_like(&reader, &mut publication)
            .await?;

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.base().events()?)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn valid() {
        let c = mocks::container();
        let uc = Like::new(
            c.event_pub(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::published_publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        uc.exec(
            reader.base().id().value().to_owned(),
            publication.base().id().value().to_owned(),
        )
        .await
        .unwrap();

        let publication = c
            .publication_repo()
            .find_by_id(&publication.base().id())
            .await
            .unwrap();
        assert_eq!(publication.statistics().likes(), 1);
        assert!(c.event_pub().events().await.len() > 0);
    }

    #[tokio::test]
    async fn not_published() {
        let c = mocks::container();
        let uc = Like::new(
            c.event_pub(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        assert!(uc
            .exec(
                reader.base().id().value().to_owned(),
                publication.base().id().value().to_owned()
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn invalid_ids() {
        let c = mocks::container();
        let uc = Like::new(
            c.event_pub(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::published_publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        assert!(uc
            .exec(reader.base().id().value().to_owned(), "#invalid".to_owned())
            .await
            .is_err());
        assert!(uc
            .exec(
                "#invalid".to_owned(),
                publication.base().id().value().to_owned()
            )
            .await
            .is_err());
    }
}
