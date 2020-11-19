use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::author::AuthorRepository;
use crate::domain::publication::{PublicationId, PublicationRepository};

pub struct Publish<'a> {
    event_pub: &'a dyn EventPublisher,

    author_repo: &'a dyn AuthorRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> Publish<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        author_repo: &'a dyn AuthorRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        Publish {
            event_pub,
            author_repo,
            publication_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        publication_id: String,
    ) -> Result<CommandResponse> {
        if !auth_role.can("publish_publication") {
            return Err(Error::unauthorized());
        }

        self.author_repo.find_by_id(&auth_id).await?;

        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        if publication.author_id() != &auth_id {
            return Err(Error::not_owner("publication"));
        }

        publication.publish()?;

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

    use identity::mocks as identity_mocks;

    use crate::mocks;

    #[tokio::test]
    async fn publish() {
        let c = mocks::container();
        let uc = Publish::new(c.event_pub(), c.author_repo(), c.publication_repo());

        let mut author = mocks::author("#user01", "user");
        c.author_repo().save(&mut author).await.unwrap();
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

        uc.exec(
            (author.base().id().clone(), role),
            publication.base().id().to_string(),
        )
        .await
        .unwrap();

        let publication = c
            .publication_repo()
            .find_by_id(&publication.base().id())
            .await
            .unwrap();
        assert_eq!(
            publication.status_history().current().to_string(),
            "waiting-approval"
        );
    }
}
