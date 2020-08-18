use std::convert::Infallible;
use std::sync::Arc;

use warp::Filter;

use catalogue::container::Container as CatalogueContainer;
use catalogue::infrastructure::persistence::inmem::InMemCatalogueRepository;
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
    pub identity: IdentityContainer<
        InMemEventBus,
        InMemRoleRepository,
        InMemTokenRepository,
        InMemUserRepository,
        BcryptHasher,
        JWTEncoder,
    >,
    pub publishing: PublishingContainer<
        InMemEventBus,
        AuthorTranslator<InMemUserRepository>,
        InMemCategoryRepository,
        InMemCollectionRepository,
        ContentManagerTranslator<InMemUserRepository>,
        InMemInteractionRepository,
        InMemPublicationRepository,
        ReaderTranslator<InMemUserRepository>,
    >,
    pub catalogue: CatalogueContainer<InMemEventBus, InMemCatalogueRepository>,
}

impl Container {
    pub fn new() -> Self {
        let event_bus = Arc::new(InMemEventBus::new());
        let event_repo = Arc::new(EventRepository::new());

        let role_repo = Arc::new(InMemRoleRepository::new());
        let token_repo = Arc::new(InMemTokenRepository::new());
        let user_repo = Arc::new(InMemUserRepository::new());
        let password_hasher = Arc::new(BcryptHasher::new());
        let token_enc = Arc::new(JWTEncoder::new());

        let _author_repo = Arc::new(InMemAuthorRepository::new());
        let author_repo = Arc::new(AuthorTranslator::new(Arc::clone(&user_repo)));
        let category_repo = Arc::new(InMemCategoryRepository::new());
        let collection_repo = Arc::new(InMemCollectionRepository::new());
        let _content_manager_repo = Arc::new(InMemContentManagerRepository::new());
        let content_manager_repo = Arc::new(ContentManagerTranslator::new(Arc::clone(&user_repo)));
        let interaction_repo = Arc::new(InMemInteractionRepository::new());
        let publication_repo = Arc::new(InMemPublicationRepository::new());
        let _reader_repo = Arc::new(InMemReaderRepository::new());
        let reader_repo = Arc::new(ReaderTranslator::new(Arc::clone(&user_repo)));

        let catalogue_repo = Arc::new(InMemCatalogueRepository::new());

        Container {
            event_bus: Arc::clone(&event_bus),
            event_repo,
            identity: IdentityContainer::new(
                Arc::clone(&event_bus),
                role_repo,
                token_repo,
                user_repo,
                password_hasher,
                token_enc,
            ),
            publishing: PublishingContainer::new(
                Arc::clone(&event_bus),
                author_repo,
                category_repo,
                collection_repo,
                content_manager_repo,
                interaction_repo,
                publication_repo,
                reader_repo,
            ),
            catalogue: CatalogueContainer::new(Arc::clone(&event_bus), catalogue_repo),
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
