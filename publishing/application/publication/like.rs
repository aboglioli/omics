use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::interaction::InteractionRepository;
use crate::domain::publication::{PublicationId, PublicationRepository};
use crate::domain::reader::ReaderRepository;

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

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        publication_id: String,
    ) -> Result<CommandResponse> {
        if !auth_role.can("like_unlike_publication") {
            return Err(Error::unauthorized());
        }

        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let reader = self.reader_repo.find_by_id(&auth_id).await?;

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

    use identity::domain::user::UserId;
    use identity::mocks as identity_mocks;

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
            "category-1",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();
        let role = identity_mocks::role("User");

        uc.exec(
            (reader.base().id().clone(), role),
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
            "category-1",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            false,
            false,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();
        let role = identity_mocks::role("User");

        assert!(uc
            .exec(
                (reader.base().id().clone(), role),
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
            "category-1",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();
        let role = identity_mocks::role("User");

        assert!(uc
            .exec(
                (reader.base().id().clone(), role.clone()),
                "#invalid".to_owned()
            )
            .await
            .is_err());
        assert!(uc
            .exec(
                (UserId::new("#invalid").unwrap(), role),
                publication.base().id().to_string()
            )
            .await
            .is_err());
    }
}
