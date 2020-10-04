use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::interaction::InteractionRepository;
use crate::domain::publication::{PublicationId, PublicationRepository};
use crate::domain::reader::{ReaderId, ReaderRepository};

pub struct Like<'a> {
    event_pub: &'a dyn EventPublisher,

    interaction_repo: &'a dyn InteractionRepository,
    publication_repo: &'a dyn PublicationRepository,
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> Like<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        interaction_repo: &'a dyn InteractionRepository,
        publication_repo: &'a dyn PublicationRepository,
        reader_repo: &'a dyn ReaderRepository,
    ) -> Self {
        Like {
            event_pub,
            interaction_repo,
            publication_repo,
            reader_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, publication_id: String) -> Result<CommandResponse> {
        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let reader_id = ReaderId::new(auth_id)?;
        let reader = self.reader_repo.find_by_id(&reader_id).await?;

        let mut like = publication.like(&reader)?;

        self.interaction_repo.save_like(&mut like).await?;
        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
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
            c.interaction_repo(),
            c.publication_repo(),
            c.reader_repo(),
        );

        let mut reader = mocks::reader("#user02", "user-2");
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::publication(
            "#publication01",
            "#user",
            "Publication 01",
            "#category01",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();

        uc.exec(
            reader.base().id().to_string(),
            publication.base().id().to_string(),
        )
        .await
        .unwrap();

        let publication = c
            .publication_repo()
            .find_by_id(&publication.base().id())
            .await
            .unwrap();
        assert_eq!(publication.statistics().likes(), 1);
        assert!(!c.event_pub().events().await.is_empty());
    }

    #[tokio::test]
    async fn not_published() {
        let c = mocks::container();
        let uc = Like::new(
            c.event_pub(),
            c.interaction_repo(),
            c.publication_repo(),
            c.reader_repo(),
        );

        let mut reader = mocks::reader("#user02", "user-2");
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::publication(
            "#publication01",
            "#user01",
            "Publication 01",
            "#category01",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            false,
            false,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();

        assert!(uc
            .exec(
                reader.base().id().to_string(),
                publication.base().id().to_string()
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn invalid_ids() {
        let c = mocks::container();
        let uc = Like::new(
            c.event_pub(),
            c.interaction_repo(),
            c.publication_repo(),
            c.reader_repo(),
        );

        let mut reader = mocks::reader("#user02", "user-2");
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::publication(
            "#publication01",
            "#user01",
            "Publication 01",
            "#category01",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();

        assert!(uc
            .exec(reader.base().id().to_string(), "#invalid".to_owned())
            .await
            .is_err());
        assert!(uc
            .exec("#invalid".to_owned(), publication.base().id().to_string())
            .await
            .is_err());
    }
}
