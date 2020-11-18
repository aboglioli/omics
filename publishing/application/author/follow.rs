use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use common::error::Error;
use identity::UserIdAndRole;

use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::interaction::InteractionRepository;
use crate::domain::reader::{ReaderId, ReaderRepository};

pub struct Follow<'a> {
    event_pub: &'a dyn EventPublisher,

    author_repo: &'a dyn AuthorRepository,
    interaction_repo: &'a dyn InteractionRepository,
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> Follow<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        author_repo: &'a dyn AuthorRepository,
        interaction_repo: &'a dyn InteractionRepository,
        reader_repo: &'a dyn ReaderRepository,
    ) -> Self {
        Follow {
            event_pub,
            author_repo,
            interaction_repo,
            reader_repo,
        }
    }

    pub async fn exec(&self, (auth_id, auth_role): UserIdAndRole, author_id: String) -> Result<CommandResponse> {
        if !auth_role.can("follow_author") {
            return Err(Error::unauthorized());
        }

        let reader = self
            .reader_repo
            .find_by_id(&auth_id)
            .await?;
        let mut author = self
            .author_repo
            .find_by_id(&AuthorId::new(author_id)?)
            .await?;

        let mut follow = author.follow(&reader)?;

        self.interaction_repo.save_follow(&mut follow).await?;
        self.author_repo.save(&mut author).await?;

        self.event_pub
            .publish_all(author.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
