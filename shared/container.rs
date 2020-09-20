use std::sync::Arc;

use async_trait::async_trait;

use common::container::Container;
use common::event::{EventPublisher, EventSubscriber};
use common::result::Result;

use crate::application::user::SyncHandler;
use crate::domain::user::{UserRepository, UserService};

pub struct SharedContainer<EPub> {
    event_pub: Arc<EPub>,

    user_repo: Arc<dyn UserRepository>,

    user_serv: Arc<dyn UserService>,
}

impl<EPub> SharedContainer<EPub>
where
    EPub: EventPublisher,
{
    pub fn new(
        event_pub: Arc<EPub>,
        user_repo: Arc<dyn UserRepository>,
        user_serv: Arc<dyn UserService>,
    ) -> Self {
        SharedContainer {
            event_pub,
            user_repo,
            user_serv,
        }
    }

    pub fn event_pub(&self) -> &EPub {
        &self.event_pub
    }

    pub fn user_repo(&self) -> &dyn UserRepository {
        self.user_repo.as_ref()
    }

    pub fn user_serv(&self) -> &dyn UserService {
        self.user_serv.as_ref()
    }
}

#[async_trait]
impl<EPub> Container for SharedContainer<EPub>
where
    EPub: Sync + Send,
{
    async fn subscribe<ES>(&self, event_sub: &ES) -> Result<()>
    where
        ES: EventSubscriber + Sync + Send,
    {
        let sync_handler = SyncHandler::new(self.user_repo.clone(), self.user_serv.clone());
        event_sub.subscribe(Box::new(sync_handler)).await?;

        Ok(())
    }

    async fn start(&self) -> Result<()> {
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }
}
