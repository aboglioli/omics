use std::sync::Arc;

use tokio_postgres::NoTls;

use common::config::Config;
use common::container::Container;
use common::event::EventSubscriber;
use common::infrastructure::event::{InMemEventBus, InMemEventRepository};
use common::result::Result;
use identity::container::IdentityContainer;
use identity::infrastructure::persistence::inmem::InMemTokenRepository;
use identity::infrastructure::persistence::postgres::{
    PostgresRoleRepository, PostgresUserRepository,
};
use identity::infrastructure::service::{BcryptHasher, JWTEncoder};
use publishing::container::PublishingContainer;

use publishing::infrastructure::persistence::postgres::{
    PostgresAuthorRepository, PostgresCategoryRepository, PostgresCollectionRepository,
    PostgresInteractionRepository, PostgresPublicationRepository, PostgresReaderRepository,
};
use shared::container::SharedContainer;
use shared::infrastructure::persistence::inmem::InMemUserRepository as SharedInMemUserRepository;

use crate::development::EventLogger;
use crate::infrastructure::shared::LocalUserService;

pub struct MainContainer {
    pub event_bus: Arc<InMemEventBus>,
    pub event_repo: Arc<InMemEventRepository>,
    pub shared: SharedContainer<InMemEventBus>,
    pub identity: IdentityContainer<InMemEventBus>,
    pub publishing: PublishingContainer<InMemEventBus>,
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
        let event_repo = Arc::new(InMemEventRepository::new());

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

        // Shared
        let s_user_repo = Arc::new(SharedInMemUserRepository::new());
        let s_user_serv = Arc::new(LocalUserService::new(i_user_repo.clone()));

        // Containers
        let identity = IdentityContainer::new(
            event_bus.clone(),
            i_role_repo,
            i_token_repo,
            i_user_repo,
            i_password_hasher,
            i_token_enc,
        );

        let publishing = PublishingContainer::new(
            event_bus.clone(),
            p_author_repo,
            p_category_repo,
            p_collection_repo,
            p_interaction_repo,
            p_publication_repo,
            p_reader_repo,
            s_user_repo.clone(),
            s_user_serv.clone(),
        );

        let shared = SharedContainer::new(event_bus.clone(), s_user_repo, s_user_serv);

        MainContainer {
            event_bus,
            event_repo,
            shared,
            identity,
            publishing,
        }
    }

    pub async fn subscribe(&self) -> Result<()> {
        let event_logger = EventLogger::new(self.event_repo.clone());
        self.event_bus.subscribe(Box::new(event_logger)).await?;

        self.shared.subscribe(self.event_bus.as_ref()).await?;
        self.identity.subscribe(self.event_bus.as_ref()).await?;
        self.publishing.subscribe(self.event_bus.as_ref()).await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn event_bus(&self) -> &InMemEventBus {
        &self.event_bus
    }

    pub fn event_repo(&self) -> &InMemEventRepository {
        &self.event_repo
    }
}
