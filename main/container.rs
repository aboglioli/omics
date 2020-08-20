use std::convert::Infallible;
use std::sync::Arc;

use warp::Filter;

use catalogue::container::Container as CatalogueContainer;
use catalogue::infrastructure::persistence::inmem::InMemCatalogueRepository;
use catalogue::infrastructure::service::{SyncCollectionService, SyncPublicationService};
use common::event::inmem::InMemEventBus;
use identity::container::Container as IdentityContainer;

use identity::infrastructure::persistence::inmem::{
    InMemRoleRepository, InMemTokenRepository, InMemUserRepository,
};
use identity::infrastructure::service::{BcryptHasher, JWTEncoder};
use publishing::container::Container as PublishingContainer;

use publishing::infrastructure::persistence::inmem::{
    InMemAuthorRepository, InMemCategoryRepository, InMemCollectionRepository,
    InMemContentManagerRepository, InMemInteractionRepository, InMemPublicationRepository,
    InMemReaderRepository,
};

use crate::infrastructure::event::EventRepository;
use crate::infrastructure::publishing::{
    AuthorTranslator, ContentManagerTranslator, ReaderTranslator,
};

pub struct Container {
    pub event_bus: Arc<InMemEventBus>,
    pub event_repo: Arc<EventRepository>,
    pub identity: IdentityContainer<InMemEventBus>,
    pub publishing: PublishingContainer<InMemEventBus>,
    pub catalogue: CatalogueContainer<InMemEventBus>,
}

impl Container {
    pub async fn new() -> Self {
        let event_bus = Arc::new(InMemEventBus::new());
        let event_repo = Arc::new(EventRepository::new());

        let role_repo = Arc::new(InMemRoleRepository::new());
        let token_repo = Arc::new(InMemTokenRepository::new());
        let user_repo = Arc::new(InMemUserRepository::new());
        let password_hasher = Arc::new(BcryptHasher::new());
        let token_enc = Arc::new(JWTEncoder::new());

        let _author_repo = Arc::new(InMemAuthorRepository::new());
        let category_repo = Arc::new(InMemCategoryRepository::new());
        let collection_repo = Arc::new(InMemCollectionRepository::new());
        let _content_manager_repo = Arc::new(InMemContentManagerRepository::new());
        let interaction_repo = Arc::new(InMemInteractionRepository::new());
        let publication_repo = Arc::new(InMemPublicationRepository::new());
        let _reader_repo = Arc::new(InMemReaderRepository::new());

        let reader_repo = Arc::new(ReaderTranslator::new(user_repo.clone()));
        let author_repo = Arc::new(AuthorTranslator::new(
            publication_repo.clone(),
            user_repo.clone(),
        ));
        let content_manager_repo = Arc::new(ContentManagerTranslator::new(user_repo.clone()));

        let catalogue_repo = Arc::new(InMemCatalogueRepository::new());
        let collection_serv = Arc::new(SyncCollectionService::new(
            author_repo.clone(),
            category_repo.clone(),
            collection_repo.clone(),
        ));
        let publication_serv = Arc::new(SyncPublicationService::new(
            author_repo.clone(),
            category_repo.clone(),
            publication_repo.clone(),
        ));

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

        let catalogue = CatalogueContainer::new(
            event_bus.clone(),
            catalogue_repo,
            collection_serv,
            publication_serv,
        );
        catalogue.subscribe(event_bus.as_ref()).await.unwrap();

        Container {
            event_bus,
            event_repo,
            identity,
            publishing,
            catalogue,
        }
    }

    pub fn event_bus(&self) -> &InMemEventBus {
        &self.event_bus
    }

    pub fn event_repo(&self) -> Arc<EventRepository> {
        Arc::clone(&self.event_repo)
    }
}

pub fn with_container(
    container: Arc<Container>,
) -> impl Filter<Extract = (Arc<Container>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&container))
}
