use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::author::AuthorId;
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

        publication.delete(&AuthorId::new(auth_id)?)?;

        self.publication_repo.save(&mut publication).await?;

        self.event_pub
            .publish_all(publication.base().events()?)
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

        let author = mocks::author1();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        assert!(uc
            .exec(
                author.base().id().to_string(),
                publication.base().id().to_string()
            )
            .await
            .is_ok());

        let publication = c
            .publication_repo()
            .find_by_id(&publication.base().id())
            .await
            .unwrap();
        assert!(publication.base().deleted_at().is_some());
    }

    #[tokio::test]
    async fn invalid() {
        let c = mocks::container();
        let uc = Delete::new(c.event_pub(), c.publication_repo());

        let author = mocks::author1();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        assert!(uc
            .exec(
                author.base().id().to_string(),
                "#invalid-publication".to_owned()
            )
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
