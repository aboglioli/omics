use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::interaction::InteractionService;
use crate::domain::reader::{ReaderId, ReaderRepository};

pub struct Unfollow<'a> {
    event_pub: &'a dyn EventPublisher,

    author_repo: &'a dyn AuthorRepository,
    reader_repo: &'a dyn ReaderRepository,

    interaction_serv: &'a InteractionService,
}

impl<'a> Unfollow<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        author_repo: &'a dyn AuthorRepository,
        reader_repo: &'a dyn ReaderRepository,
        interaction_serv: &'a InteractionService,
    ) -> Self {
        Unfollow {
            event_pub,
            author_repo,
            reader_repo,
            interaction_serv,
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

        self.interaction_serv
            .delete_follow(&reader, &mut author)
            .await?;

        self.author_repo.save(&mut author).await?;

        self.event_pub
            .publish_all(author.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}
