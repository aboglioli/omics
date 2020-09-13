use std::sync::Arc;

use async_trait::async_trait;

use common::error::Error;
use common::event::{Event, EventHandler};
use common::result::Result;
use shared::event::UserEvent;

use crate::domain::author::{Author, AuthorId, AuthorRepository};

pub struct AuthorFromUserHandler {
    author_repo: Arc<dyn AuthorRepository>,
}

impl AuthorFromUserHandler {
    pub fn new(author_repo: Arc<dyn AuthorRepository>) -> Self {
        AuthorFromUserHandler { author_repo }
    }
}

#[async_trait]
impl EventHandler for AuthorFromUserHandler {
    fn topic(&self) -> &str {
        "user"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event: UserEvent = serde_json::from_slice(event.payload())
            .map_err(|err| Error::new("author_from_user_handler", "deserialize").wrap_raw(err))?;

        match event {
            UserEvent::Validated { id } => {
                let mut author = Author::new(AuthorId::new(id)?)?;
                self.author_repo.save(&mut author).await?;
            }
            UserEvent::Deleted { id } => {
                let mut author = self.author_repo.find_by_id(&AuthorId::new(id)?).await?;
                author.delete()?;
                self.author_repo.save(&mut author).await?;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
