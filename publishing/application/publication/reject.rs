use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::interaction::Comment;
use crate::domain::publication::{PublicationId, PublicationRepository};
use crate::domain::user::{UserId, UserRepository};

#[derive(Deserialize)]
pub struct RejectCommand {
    pub comment: String,
}

pub struct Reject<'a> {
    event_pub: &'a dyn EventPublisher,

    publication_repo: &'a dyn PublicationRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Reject<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        publication_repo: &'a dyn PublicationRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Reject {
            event_pub,
            publication_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        auth_id: String,
        publication_id: String,
        cmd: RejectCommand,
    ) -> Result<CommandResponse> {
        let user_id = UserId::new(auth_id)?;
        let user = self.user_repo.find_by_id(&user_id).await?;

        if !user.is_content_manager() {
            return Err(Error::unauthorized());
        }

        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let comment = Comment::new(cmd.comment)?;

        publication.reject(user_id, comment)?;

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

    use crate::domain::publication::Status;
    use crate::mocks;

    #[tokio::test]
    async fn reject() {
        let c = mocks::container();
        let uc = Reject::new(c.event_pub(), c.publication_repo(), c.user_repo());

        let (mut user1, mut author1, mut reader1) = mocks::user1();
        c.user_repo().save(&mut user1).await.unwrap();
        c.author_repo().save(&mut author1).await.unwrap();
        c.reader_repo().save(&mut reader1).await.unwrap();

        let (mut cm, _, _) = mocks::content_manager1();
        c.user_repo().save(&mut cm).await.unwrap();

        let mut publication = mocks::publication1();
        publication.publish().unwrap();
        c.publication_repo().save(&mut publication).await.unwrap();

        uc.exec(
            cm.base().id().to_string(),
            publication.base().id().to_string(),
            RejectCommand {
                comment: "All is OK".to_owned(),
            },
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
            "rejected"
        );

        if let Status::Rejected { admin_id, comment } = publication.status_history().current() {
            assert_eq!(admin_id, cm.base().id());
            assert_eq!(comment.value(), "All is OK");
        }
    }
}
