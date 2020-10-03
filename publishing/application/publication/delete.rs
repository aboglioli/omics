use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::publication::{PublicationId, PublicationRepository};

pub struct Delete<'a> {
    event_pub: &'a dyn EventPublisher,

    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> Delete<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        Delete {
            event_pub,
            publication_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, publication_id: String) -> Result<CommandResponse> {
        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        if publication.author_id().value() != auth_id {
            return Err(Error::not_owner("publication"));
        }

        publication.delete()?;

        self.publication_repo
            .delete(publication.base().id())
            .await?;

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
        let uc = Delete::new(c.event_pub(), c.publication_repo());

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
        );
        c.publication_repo().save(&mut publication).await.unwrap();

        assert!(uc
            .exec("#user01".to_owned(), publication.base().id().to_string())
            .await
            .is_ok());

        assert!(c
            .publication_repo()
            .find_by_id(&publication.base().id())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn invalid() {
        let c = mocks::container();
        let uc = Delete::new(c.event_pub(), c.publication_repo());

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
        );
        c.publication_repo().save(&mut publication).await.unwrap();

        assert!(uc
            .exec("#user01".to_owned(), "#invalid-publication".to_owned())
            .await
            .is_err());
        assert!(uc
            .exec(
                "#invald-author".to_owned(),
                publication.base().id().to_string()
            )
            .await
            .is_err());
    }
}
