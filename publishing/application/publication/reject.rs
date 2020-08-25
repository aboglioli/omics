use common::event::EventPublisher;
use common::result::Result;

use crate::application::dtos::CommandResponse;
use crate::domain::content_manager::{ContentManagerId, ContentManagerRepository};
use crate::domain::publication::{PublicationId, PublicationRepository};

pub struct Reject<'a> {
    event_pub: &'a dyn EventPublisher,

    content_manager_repo: &'a dyn ContentManagerRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> Reject<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        content_manager_repo: &'a dyn ContentManagerRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        Reject {
            event_pub,
            content_manager_repo,
            publication_repo,
        }
    }

    pub async fn exec(
        &self,
        content_manager_id: String,
        publication_id: String,
    ) -> Result<CommandResponse> {
        let content_manager_id = ContentManagerId::new(content_manager_id)?;
        let content_manager = self
            .content_manager_repo
            .find_by_id(&content_manager_id)
            .await?;

        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        publication.reject(&content_manager)?;

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

    use crate::domain::publication::Status;
    use crate::mocks;

    #[tokio::test]
    async fn reject() {
        let c = mocks::container();
        let uc = Reject::new(
            c.event_pub(),
            c.content_manager_repo(),
            c.publication_repo(),
        );

        let author = mocks::author1();
        let mut cm = mocks::content_manager1();
        c.content_manager_repo().save(&mut cm).await.unwrap();
        let mut publication = mocks::publication1();
        publication.publish(&author).unwrap();
        c.publication_repo().save(&mut publication).await.unwrap();

        uc.exec(
            cm.base().id().to_string(),
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
            publication.status_history().current().status().to_string(),
            "rejected"
        );

        if let Status::Rejected { admin_id } = publication.status_history().current().status() {
            assert_eq!(admin_id, cm.base().id());
        }
    }
}
