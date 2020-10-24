use std::sync::Arc;

use async_trait::async_trait;

use common::config::Config;
use common::container::Container;
use common::event::{EventPublisher, EventSubscriber};
use common::result::Result;
use identity::domain::user::UserRepository;
use publishing::domain::author::AuthorRepository;
use publishing::domain::collection::CollectionRepository;
use publishing::domain::interaction::InteractionRepository;
use publishing::domain::publication::PublicationRepository;

use crate::application::author::ApprovedRejectedPublicationHandler;
use crate::application::notification::NotificationHandler;
use crate::application::user::RegisteredHandler;

use crate::domain::email::EmailService;
use crate::domain::notification::NotificationRepository;

pub struct NotificationContainer<EPub> {
    event_pub: Arc<EPub>,

    author_repo: Arc<dyn AuthorRepository>,
    collection_repo: Arc<dyn CollectionRepository>,
    interaction_repo: Arc<dyn InteractionRepository>,
    notification_repo: Arc<dyn NotificationRepository>,
    publication_repo: Arc<dyn PublicationRepository>,
    user_repo: Arc<dyn UserRepository>,

    email_serv: Arc<dyn EmailService>,
}

impl<EPub> NotificationContainer<EPub>
where
    EPub: EventPublisher,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        event_pub: Arc<EPub>,
        author_repo: Arc<dyn AuthorRepository>,
        collection_repo: Arc<dyn CollectionRepository>,
        interaction_repo: Arc<dyn InteractionRepository>,
        notification_repo: Arc<dyn NotificationRepository>,
        publication_repo: Arc<dyn PublicationRepository>,
        user_repo: Arc<dyn UserRepository>,
        email_serv: Arc<dyn EmailService>,
    ) -> Self {
        NotificationContainer {
            event_pub,
            author_repo,
            collection_repo,
            interaction_repo,
            notification_repo,
            publication_repo,
            user_repo,
            email_serv,
        }
    }

    pub fn event_pub(&self) -> &EPub {
        &self.event_pub
    }

    pub fn author_repo(&self) -> &dyn AuthorRepository {
        self.author_repo.as_ref()
    }

    pub fn collection_repo(&self) -> &dyn CollectionRepository {
        self.collection_repo.as_ref()
    }

    pub fn interaction_repo(&self) -> &dyn InteractionRepository {
        self.interaction_repo.as_ref()
    }

    pub fn notification_repo(&self) -> &dyn NotificationRepository {
        self.notification_repo.as_ref()
    }

    pub fn publication_repo(&self) -> &dyn PublicationRepository {
        self.publication_repo.as_ref()
    }

    pub fn user_repo(&self) -> &dyn UserRepository {
        self.user_repo.as_ref()
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
        let config = Config::get();

        if config.env() == "production" {
            let registered_handler = RegisteredHandler::new(self.email_serv.clone());
            event_sub.subscribe(Box::new(registered_handler)).await?;

            let approved_rejected_publication_handler = ApprovedRejectedPublicationHandler::new(
                self.publication_repo.clone(),
                self.user_repo.clone(),
                self.email_serv.clone(),
            );
            event_sub
                .subscribe(Box::new(approved_rejected_publication_handler))
                .await?;
        }

        let notification_handler = NotificationHandler::new(
            self.author_repo.clone(),
            self.interaction_repo.clone(),
            self.notification_repo.clone(),
            self.publication_repo.clone(),
            self.user_repo.clone(),
        );
        event_sub.subscribe(Box::new(notification_handler)).await?;

        Ok(())
    }
}
