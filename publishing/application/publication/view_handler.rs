use async_trait::async_trait;

use common::error::Error;
use common::event::{Event, EventHandler};
use common::result::Result;
use shared::domain::event::PublicationEvent;

use crate::domain::interaction::{InteractionRepository, View};
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

pub struct ViewHandler<'a, IRepo> {
    interaction_repo: &'a IRepo,
}

#[async_trait]
impl<'a, IRepo> EventHandler for ViewHandler<'a, IRepo>
where
    IRepo: InteractionRepository + Sync + Send,
{
    type Output = bool;

    fn topic(&self) -> &str {
        "user"
    }

    async fn handle(&mut self, event: &Event) -> Result<Self::Output> {
        if event.code() != "viewed" {
            return Ok(false);
        }

        let event: PublicationEvent = match serde_json::from_slice(event.payload()) {
            Ok(event) => event,
            Err(err) => {
                return Err(Error::new("view_handler", "deserialize")
                    .wrap_raw(err)
                    .build())
            }
        };

        if let PublicationEvent::Viewed {
            reader_id,
            publication_id,
        } = event
        {
            let mut view = View::new(
                ReaderId::new(&reader_id)?,
                PublicationId::new(&publication_id)?,
            )?;

            self.interaction_repo.save_view(&mut view).await?;

            return Ok(true);
        }

        Ok(false)
    }
}
