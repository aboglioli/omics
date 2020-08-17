use std::sync::Arc;

use common::event::EventPublisher;

use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::CollectionRepository;
use crate::domain::content_manager::ContentManagerRepository;
use crate::domain::interaction::{InteractionRepository, InteractionService};
use crate::domain::publication::{PublicationRepository, StatisticsService};
use crate::domain::reader::ReaderRepository;

pub struct Container<EPub, ARepo, CatRepo, CollRepo, CMRepo, IRepo, PRepo, RRepo> {
    event_pub: Arc<EPub>,

    author_repo: Arc<ARepo>,
    category_repo: Arc<CatRepo>,
    collection_repo: Arc<CollRepo>,
    content_manager_repo: Arc<CMRepo>,
    interaction_repo: Arc<IRepo>,
    publication_repo: Arc<PRepo>,
    reader_repo: Arc<RRepo>,

    statistics_serv: Arc<StatisticsService<IRepo>>,
    interaction_serv: Arc<InteractionService<IRepo>>,
}

impl<EPub, ARepo, CatRepo, CollRepo, CMRepo, IRepo, PRepo, RRepo>
    Container<EPub, ARepo, CatRepo, CollRepo, CMRepo, IRepo, PRepo, RRepo>
where
    EPub: EventPublisher,
    ARepo: AuthorRepository,
    CatRepo: CategoryRepository,
    CollRepo: CollectionRepository,
    CMRepo: ContentManagerRepository,
    IRepo: InteractionRepository,
    PRepo: PublicationRepository,
    RRepo: ReaderRepository,
{
    pub fn new(
        event_pub: Arc<EPub>,
        author_repo: Arc<ARepo>,
        category_repo: Arc<CatRepo>,
        collection_repo: Arc<CollRepo>,
        content_manager_repo: Arc<CMRepo>,
        interaction_repo: Arc<IRepo>,
        publication_repo: Arc<PRepo>,
        reader_repo: Arc<RRepo>,
    ) -> Self {
        let statistics_serv = Arc::new(StatisticsService::new(Arc::clone(&interaction_repo)));
        let interaction_serv = Arc::new(InteractionService::new(Arc::clone(&interaction_repo)));

        Container {
            event_pub,

            author_repo,
            category_repo,
            collection_repo,
            content_manager_repo,
            interaction_repo,
            publication_repo,
            reader_repo,

            statistics_serv,
            interaction_serv,
        }
    }

    pub fn event_pub(&self) -> &EPub {
        &self.event_pub
    }

    pub fn author_repo(&self) -> &ARepo {
        &self.author_repo
    }

    pub fn category_repo(&self) -> &CatRepo {
        &self.category_repo
    }

    pub fn collection_repo(&self) -> &CollRepo {
        &self.collection_repo
    }

    pub fn content_manager_repo(&self) -> &CMRepo {
        &self.content_manager_repo
    }

    pub fn interaction_repo(&self) -> &IRepo {
        &self.interaction_repo
    }

    pub fn publication_repo(&self) -> &PRepo {
        &self.publication_repo
    }

    pub fn reader_repo(&self) -> &RRepo {
        &self.reader_repo
    }

    // Service
    pub fn statistics_serv(&self) -> &StatisticsService<IRepo> {
        &self.statistics_serv
    }

    pub fn interaction_serv(&self) -> &InteractionService<IRepo> {
        &self.interaction_serv
    }
}
