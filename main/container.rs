use std::sync::Arc;

use common::event::EventSubscriber;
use common::infrastructure::event::{InMemEventBus, InMemEventRepository};
use common::result::Result;
use identity::container::Container as IdentityContainer;
use identity::infrastructure::persistence::inmem::{
    InMemRoleRepository, InMemTokenRepository, InMemUserRepository,
};
use identity::infrastructure::service::{BcryptHasher, JWTEncoder};
use publishing::container::Container as PublishingContainer;
use publishing::infrastructure::persistence::inmem::{
    InMemAuthorRepository, InMemCategoryRepository, InMemCollectionRepository,
    InMemInteractionRepository, InMemPublicationRepository, InMemReaderRepository,
    InMemUserRepository as PInMemUserRepository,
};

use crate::development::EventLogger;
use crate::infrastructure::publishing::LocalUserService;

pub struct Container {
    pub event_bus: Arc<InMemEventBus>,
    pub event_repo: Arc<InMemEventRepository>,
    pub identity: IdentityContainer<InMemEventBus>,
    pub publishing: PublishingContainer<InMemEventBus>,
}

impl Container {
    pub async fn new() -> Self {
        // Common
        let event_bus = Arc::new(InMemEventBus::new());
        let event_repo = Arc::new(InMemEventRepository::new());

        // Identity
        let i_role_repo = Arc::new(InMemRoleRepository::new());
        let i_token_repo = Arc::new(InMemTokenRepository::new());
        let i_user_repo = Arc::new(InMemUserRepository::new());
        let i_password_hasher = Arc::new(BcryptHasher::new());
        let i_token_enc = Arc::new(JWTEncoder::new());

        // Publishing
        let p_author_repo = Arc::new(InMemAuthorRepository::new());
        let p_category_repo = Arc::new(InMemCategoryRepository::new());
        let p_collection_repo = Arc::new(InMemCollectionRepository::new());
        let p_interaction_repo = Arc::new(InMemInteractionRepository::new());
        let p_publication_repo = Arc::new(InMemPublicationRepository::new());
        let p_reader_repo = Arc::new(InMemReaderRepository::new());
        let p_user_repo = Arc::new(PInMemUserRepository::new());
        let p_user_serv = Arc::new(LocalUserService::new(i_user_repo.clone()));

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
            p_user_repo,
            p_user_serv,
        );

        Container {
            event_bus,
            event_repo,
            identity,
            publishing,
        }
    }

    pub async fn subscribe(&self) -> Result<()> {
        let event_logger = EventLogger::new(self.event_repo.clone());
        self.event_bus.subscribe(Box::new(event_logger)).await?;

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
