use std::sync::Arc;

use common::event::EventSubscriber;
use common::infrastructure::event::{InMemEventBus, InMemEventRepository};
use common::result::Result;
use identity::container::Container as IdentityContainer;
use identity::infrastructure::persistence::inmem::{
    InMemRoleRepository, InMemTokenRepository, InMemUserRepository,
};
use identity::infrastructure::service::{BcryptHasher, JWTEncoder};
use publishing::application::reader::InteractionHandler;
use publishing::container::Container as PublishingContainer;
use publishing::infrastructure::persistence::inmem::{
    InMemCategoryRepository, InMemCollectionRepository, InMemInteractionRepository,
    InMemPublicationRepository,
};

use crate::development::EventLogger;
use crate::infrastructure::publishing::{
    AuthorTranslator, ContentManagerTranslator, ReaderTranslator,
};

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
        let role_repo = Arc::new(InMemRoleRepository::new());
        let token_repo = Arc::new(InMemTokenRepository::new());
        let user_repo = Arc::new(InMemUserRepository::new());
        let password_hasher = Arc::new(BcryptHasher::new());
        let token_enc = Arc::new(JWTEncoder::new());

        // Publishing
        let category_repo = Arc::new(InMemCategoryRepository::new());
        let collection_repo = Arc::new(InMemCollectionRepository::new());
        let interaction_repo = Arc::new(InMemInteractionRepository::new());
        let publication_repo = Arc::new(InMemPublicationRepository::new());

        let author_repo = Arc::new(AuthorTranslator::new(user_repo.clone()));
        let content_manager_repo = Arc::new(ContentManagerTranslator::new(user_repo.clone()));
        let reader_repo = Arc::new(ReaderTranslator::new(user_repo.clone()));

        // Containers
        let identity = IdentityContainer::new(
            event_bus.clone(),
            role_repo,
            token_repo,
            user_repo,
            password_hasher,
            token_enc,
        );

        let publishing = PublishingContainer::new(
            event_bus.clone(),
            author_repo,
            category_repo,
            collection_repo,
            content_manager_repo,
            interaction_repo,
            publication_repo,
            reader_repo,
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

        let reader_handler = InteractionHandler::new(
            self.publishing.reader_repo_clone(),
            self.publishing.publication_repo_clone(),
        );
        self.event_bus.subscribe(Box::new(reader_handler)).await?;

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
