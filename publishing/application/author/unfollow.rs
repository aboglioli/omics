use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::interaction::InteractionRepository;
use crate::domain::reader::{ReaderId, ReaderRepository};

pub struct Unfollow<'a> {
    event_pub: &'a dyn EventPublisher,

    author_repo: &'a dyn AuthorRepository,
    interaction_repo: &'a dyn InteractionRepository,
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> Unfollow<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        author_repo: &'a dyn AuthorRepository,
        interaction_repo: &'a dyn InteractionRepository,
        reader_repo: &'a dyn ReaderRepository,
    ) -> Self {
        Unfollow {
            event_pub,
            author_repo,
            interaction_repo,
            reader_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, author_id: String) -> Result<CommandResponse> {
        let reader = self
            .reader_repo
            .find_by_id(&ReaderId::new(auth_id)?)
            .await?;
        let mut author = self
            .author_repo
            .find_by_id(&AuthorId::new(author_id)?)
            .await?;

        author.unfollow(&reader)?;

        self.interaction_repo
            .delete_follow(reader.base().id(), author.base().id())
            .await?;
        self.author_repo.save(&mut author).await?;

        self.event_pub
            .publish_all(author.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
