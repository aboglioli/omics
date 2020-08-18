use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;

use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::publication::{PublicationId, PublicationRepository};

// TODO: add comment
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

    pub async fn exec(&self, author_id: String, publication_id: String) -> Result<()> {
        let author_id = AuthorId::new(author_id)?;
        self.author_repo.find_by_id(&author_id).await?;

        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        if publication.author_id() != &author_id {
            return Err(Error::new("publication", "unauthorized"));
        }

        publication.publish()?;

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

    use crate::domain::publication::Status;
    use crate::mocks;

    #[tokio::test]
    async fn publish() {
        let c = mocks::container();
        let uc = Publish::new(c.event_pub(), c.author_repo(), c.publication_repo());

        let mut author = mocks::author1();
        c.author_repo().save(&mut author).await.unwrap();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        uc.exec(
            author.base().id().value().to_owned(),
            publication.base().id().value().to_owned(),
        )
        .await
        .unwrap();

        let publication = c
            .publication_repo()
            .find_by_id(&publication.base().id())
            .await
            .unwrap();
        assert_eq!(
            publication.status_history().current().status().to_string(),
            "waiting-approval"
        );

        if let Status::Published { admin_id } = publication.status_history().current().status() {
            assert_eq!(admin_id, &author.base().id());
        }
    }
}
