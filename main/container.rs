use std::sync::Arc;

use tokio_postgres::NoTls;

use common::config::Config;
use common::container::Container;
use common::event::EventSubscriber;
use common::infrastructure::event::{InMemEventBus, PostgresEventRepository};
use common::result::Result;
use identity::container::IdentityContainer;
use identity::infrastructure::persistence::inmem::InMemTokenRepository;
use identity::infrastructure::persistence::postgres::{
    PostgresRoleRepository, PostgresUserRepository,
};
use identity::infrastructure::service::{BcryptHasher, JWTEncoder};
use notification::container::NotificationContainer;
use notification::infrastructure::persistence::postgres::PostgresNotificationRepository;
use notification::infrastructure::service::GmailService;
use publishing::container::PublishingContainer;
use publishing::infrastructure::persistence::postgres::{
    PostgresAuthorRepository, PostgresCategoryRepository, PostgresCollectionRepository,
    PostgresInteractionRepository, PostgresPublicationRepository, PostgresReaderRepository,
};

use crate::development::EventLogger;

pub struct MainContainer {
    pub event_bus: Arc<InMemEventBus>,
    pub event_repo: Arc<PostgresEventRepository>,
    pub identity: IdentityContainer<InMemEventBus>,
    pub publishing: PublishingContainer<InMemEventBus>,
    pub notification: NotificationContainer<InMemEventBus>,
}

impl MainContainer {
    pub async fn new() -> Self {
        let config = Config::get();
        let (client, connection) = tokio_postgres::connect(
            &format!(
                "host={} user={} password={} dbname={}",
                config.postgres_host(),
                config.postgres_username(),
                config.postgres_password(),
                config.postgres_database()
            ),
            NoTls,
        )
        .await
        .unwrap();

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                eprintln!("error: {}", err);
            }
        });

        let client = Arc::new(client);

        // Common
        let event_bus = Arc::new(InMemEventBus::new());
        let event_repo = Arc::new(PostgresEventRepository::new(client.clone()));

        // Identity
        let i_role_repo = Arc::new(PostgresRoleRepository::new(client.clone()));
        let i_token_repo = Arc::new(InMemTokenRepository::new());
        let i_user_repo = Arc::new(PostgresUserRepository::new(client.clone()));
        let i_password_hasher = Arc::new(BcryptHasher::new());
        let i_token_enc = Arc::new(JWTEncoder::new());

        // Publishing
        let p_author_repo = Arc::new(PostgresAuthorRepository::new(client.clone()));
        let p_category_repo = Arc::new(PostgresCategoryRepository::new(client.clone()));
        let p_collection_repo = Arc::new(PostgresCollectionRepository::new(client.clone()));
        let p_interaction_repo = Arc::new(PostgresInteractionRepository::new(client.clone()));
        let p_publication_repo = Arc::new(PostgresPublicationRepository::new(client.clone()));
        let p_reader_repo = Arc::new(PostgresReaderRepository::new(client.clone()));

        // Notification
        let n_notification_repo = Arc::new(PostgresNotificationRepository::new(client.clone()));
        let n_email_serv = Arc::new(GmailService::new());

        // Containers
        let identity = IdentityContainer::new(
            event_bus.clone(),
            i_role_repo,
            i_token_repo,
            i_user_repo.clone(),
            i_password_hasher,
            i_token_enc,
        );

        let publishing = PublishingContainer::new(
            event_bus.clone(),
            p_author_repo.clone(),
            p_category_repo.clone(),
            p_collection_repo.clone(),
            p_interaction_repo.clone(),
            p_publication_repo.clone(),
            p_reader_repo.clone(),
            i_user_repo.clone(),
        );

        let notification = NotificationContainer::new(
            event_bus.clone(),
            p_author_repo.clone(),
            p_collection_repo.clone(),
            p_interaction_repo.clone(),
            n_notification_repo,
            p_publication_repo.clone(),
            i_user_repo.clone(),
            n_email_serv,
        );

        MainContainer {
            event_bus,
            event_repo,
            identity,
            publishing,
            notification,
        }
    }

    pub async fn subscribe(&self) -> Result<()> {
        let event_logger = EventLogger::new(self.event_repo.clone());
        self.event_bus.subscribe(Box::new(event_logger)).await?;

        self.identity.subscribe(self.event_bus.as_ref()).await?;
        self.publishing.subscribe(self.event_bus.as_ref()).await?;
        self.notification.subscribe(self.event_bus.as_ref()).await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn event_bus(&self) -> &InMemEventBus {
        &self.event_bus
    }

    pub fn event_repo(&self) -> &PostgresEventRepository {
        &self.event_repo
    }
}
