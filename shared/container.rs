use std::sync::Arc;

use common::event::{EventPublisher, EventSubscriber};
use common::result::Result;

use crate::application::user::SyncHandler;
use crate::domain::user::{UserRepository, UserService};

pub struct Container<EPub> {
    event_pub: Arc<EPub>,

    user_repo: Arc<dyn UserRepository>,

    user_serv: Arc<dyn UserService>,
}

impl<EPub> Container<EPub>
where
    EPub: EventPublisher,
{
    pub fn new(
        event_pub: Arc<EPub>,
        user_repo: Arc<dyn UserRepository>,
        user_serv: Arc<dyn UserService>,
    ) -> Self {
        Container {
            event_pub,
            user_repo,
            user_serv,
        }
    }

    pub async fn subscribe<ES>(&self, event_sub: &ES) -> Result<()>
    where
        ES: EventSubscriber,
    {
        let sync_handler = SyncHandler::new(self.user_repo.clone(), self.user_serv.clone());
        event_sub.subscribe(Box::new(sync_handler)).await?;

        Ok(())
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
