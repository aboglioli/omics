use std::convert::Infallible;
use std::sync::Arc;

use warp::Filter;

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

pub struct Container {
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
        InMemAuthorRepository,
        InMemCategoryRepository,
        InMemCollectionRepository,
        InMemContentManagerRepository,
        InMemInteractionRepository,
        InMemPublicationRepository,
        InMemReaderRepository,
    >,
}

impl Container {
    pub fn new() -> Self {
        let event_bus = Arc::new(InMemEventBus::new());

        let role_repo = InMemRoleRepository::new();
        let token_repo = InMemTokenRepository::new();
        let user_repo = InMemUserRepository::new();
        let password_hasher = BcryptHasher::new();
        let token_enc = JWTEncoder::new();

        let author_repo = InMemAuthorRepository::new();
        let category_repo = InMemCategoryRepository::new();
        let collection_repo = InMemCollectionRepository::new();
        let content_manager_repo = InMemContentManagerRepository::new();
        let interaction_repo = InMemInteractionRepository::new();
        let publication_repo = InMemPublicationRepository::new();
        let reader_repo = InMemReaderRepository::new();

        Container {
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
        }
    }
}

pub fn with_container(
    container: Arc<Container>,
) -> impl Filter<Extract = (Arc<Container>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&container))
}
