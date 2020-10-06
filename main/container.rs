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
use payment::container::PaymentContainer;
use payment::infrastructure::persistence::postgres::{
    PostgresContractRepository, PostgresPlanRepository, PostgresSubscriptionRepository,
};
use payment::infrastructure::service::MercadoPagoService;
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
    pub payment: PaymentContainer<InMemEventBus>,
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
        let id_role_repo = Arc::new(PostgresRoleRepository::new(client.clone()));
        let id_tokenot_repo = Arc::new(InMemTokenRepository::new());
        let id_user_repo = Arc::new(PostgresUserRepository::new(client.clone()));
        let id_password_hasher = Arc::new(BcryptHasher::new());
        let id_tokenot_enc = Arc::new(JWTEncoder::new());

        // Publishing
        let pub_author_repo = Arc::new(PostgresAuthorRepository::new(client.clone()));
        let pub_category_repo = Arc::new(PostgresCategoryRepository::new(client.clone()));
        let pub_collectionot_repo = Arc::new(PostgresCollectionRepository::new(client.clone()));
        let pub_interactionot_repo = Arc::new(PostgresInteractionRepository::new(client.clone()));
        let pub_publicationot_repo = Arc::new(PostgresPublicationRepository::new(client.clone()));
        let pub_reader_repo = Arc::new(PostgresReaderRepository::new(client.clone()));

        // Payment
        let pay_contract_repo = Arc::new(PostgresContractRepository::new(client.clone()));
        let pay_plan_repo = Arc::new(PostgresPlanRepository::new(client.clone()));
        let pay_subscription_repo = Arc::new(PostgresSubscriptionRepository::new(client.clone()));
        let pay_payment_serv = Arc::new(MercadoPagoService::new());

        // Notification
        let not_notificationot_repo = Arc::new(PostgresNotificationRepository::new(client));
        let not_email_serv = Arc::new(GmailService::new());

        // Containers
        let identity = IdentityContainer::new(
            event_bus.clone(),
            id_role_repo,
            id_tokenot_repo,
            id_user_repo.clone(),
            id_password_hasher,
            id_tokenot_enc,
        );

        let publishing = PublishingContainer::new(
            event_bus.clone(),
            pub_author_repo.clone(),
            pub_category_repo,
            pub_collectionot_repo.clone(),
            pub_interactionot_repo.clone(),
            pub_publicationot_repo.clone(),
            pub_reader_repo,
            id_user_repo.clone(),
        );

        let payment = PaymentContainer::new(
            event_bus.clone(),
            pay_contract_repo,
            pay_plan_repo,
            pub_publicationot_repo.clone(),
            pay_subscription_repo,
            id_user_repo.clone(),
            pay_payment_serv,
            publishing.statistics_serv_clone(),
        );

        let notification = NotificationContainer::new(
            event_bus.clone(),
            pub_author_repo,
            pub_collectionot_repo,
            pub_interactionot_repo,
            not_notificationot_repo,
            pub_publicationot_repo,
            id_user_repo,
            not_email_serv,
        );

        MainContainer {
            event_bus,
            event_repo,
            identity,
            publishing,
            payment,
            notification,
        }
    }

    pub async fn subscribe(&self) -> Result<()> {
        let event_logger = EventLogger::new(self.event_repo.clone());
        self.event_bus.subscribe(Box::new(event_logger)).await?;

        self.identity.subscribe(self.event_bus.as_ref()).await?;
        self.publishing.subscribe(self.event_bus.as_ref()).await?;
        self.payment.subscribe(self.event_bus.as_ref()).await?;
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
