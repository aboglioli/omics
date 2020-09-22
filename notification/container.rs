use std::sync::Arc;

use async_trait::async_trait;

use common::container::Container;
use common::event::{EventPublisher, EventSubscriber};
use common::result::Result;

use crate::application::user::RegisteredHandler;
use crate::domain::email::EmailService;

pub struct NotificationContainer<EPub> {
    event_pub: Arc<EPub>,

    email_serv: Arc<dyn EmailService>,
}

impl<EPub> NotificationContainer<EPub>
where
    EPub: EventPublisher,
{
    pub fn new(event_pub: Arc<EPub>, email_serv: Arc<dyn EmailService>) -> Self {
        NotificationContainer {
            event_pub,
            email_serv,
        }
    }

    pub fn event_pub(&self) -> &EPub {
        &self.event_pub
    }

    pub fn email_serv(&self) -> &dyn EmailService {
        self.email_serv.as_ref()
    }
}

#[async_trait]
impl<EPub> Container for NotificationContainer<EPub>
where
    EPub: Sync + Send,
{
    async fn subscribe<ES>(&self, event_sub: &ES) -> Result<()>
    where
        ES: EventSubscriber + Sync + Send,
    {
        let registered_handler = RegisteredHandler::new(self.email_serv.clone());
        event_sub.subscribe(Box::new(registered_handler)).await?;

        Ok(())
    }
}
