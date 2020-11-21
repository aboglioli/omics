use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::interaction::Comment;
use crate::domain::publication::{PublicationId, PublicationRepository};

#[derive(Deserialize)]
pub struct ApproveCommand {
    pub comment: String,
}

pub struct Approve<'a> {
    event_pub: &'a dyn EventPublisher,

    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> Approve<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        Approve {
            event_pub,
            publication_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        publication_id: String,
        cmd: ApproveCommand,
    ) -> Result<CommandResponse> {
        if !auth_role.can("approve_reject_publication") {
            return Err(Error::unauthorized());
        }

        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let comment = Comment::new(cmd.comment)?;

        publication.approve(auth_id, comment)?;

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

    use crate::domain::publication::Status;
    use crate::mocks;

    #[tokio::test]
    async fn approve() {
        let c = mocks::container();
        let uc = Approve::new(c.event_pub(), c.publication_repo());

        let mut user = identity_mocks::user(
            "#content-manager01",
            "content-manager-1",
            "content-manager@omics.com",
            "P@sswd!",
            true,
            None,
            None,
            "content-manager",
        );
        c.user_repo().save(&mut user).await.unwrap();

        let mut publication = mocks::publication(
            "#publication01",
            "#user01",
            "Publication 01",
            "category-1",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            true,
            false,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();
        let role = identity_mocks::role("User");

        uc.exec(
            (user.base().id().clone(), role),
            publication.base().id().to_string(),
            ApproveCommand {
                comment: "All is OK".to_owned(),
            },
        )
        .await
        .unwrap();

        let publication = c
            .publication_repo()
            .find_by_id(publication.base().id())
            .await
            .unwrap();
        assert_eq!(
            publication.status_history().current().to_string(),
            "published"
        );

        if let Status::Published {
            admin_id: Some(admin_id),
            comment: Some(comment),
        } = publication.status_history().current()
        {
            assert_eq!(admin_id, user.base().id());
            assert_eq!(comment.value(), "All is OK");
        }
    }
}
