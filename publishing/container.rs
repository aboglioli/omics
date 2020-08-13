use std::sync::Arc;

use common::event::EventPublisher;

use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::CollectionRepository;
use crate::domain::content_manager::ContentManagerRepository;
use crate::domain::interaction::InteractionRepository;
use crate::domain::publication::PublicationRepository;
use crate::domain::reader::ReaderRepository;
use crate::domain::statistics::StatisticsRepository;

pub struct Container<EPub, ARepo, CatRepo, CollRepo, CMRepo, IRepo, PRepo, RRepo, SRepo> {
    event_pub: Arc<EPub>,

    author_repo: ARepo,
    category_repo: CatRepo,
    collection_repo: CollRepo,
    content_manager_repo: CMRepo,
    interaction_repo: IRepo,
    publication_repo: PRepo,
    reader_repo: RRepo,
    statistics_repo: SRepo,
}

impl<EPub, ARepo, CatRepo, CollRepo, CMRepo, IRepo, PRepo, RRepo, SRepo>
    Container<EPub, ARepo, CatRepo, CollRepo, CMRepo, IRepo, PRepo, RRepo, SRepo>
where
    EPub: EventPublisher,
    ARepo: AuthorRepository,
    CatRepo: CategoryRepository,
    CollRepo: CollectionRepository,
    CMRepo: ContentManagerRepository,
    IRepo: InteractionRepository,
    PRepo: PublicationRepository,
    RRepo: ReaderRepository,
    SRepo: StatisticsRepository,
{
    pub fn new(
        event_pub: Arc<EPub>,
        author_repo: ARepo,
        category_repo: CatRepo,
        collection_repo: CollRepo,
        content_manager_repo: CMRepo,
        interaction_repo: IRepo,
        publication_repo: PRepo,
        reader_repo: RRepo,
        statistics_repo: SRepo,
    ) -> Self {
        Container {
            event_pub,
            author_repo,
            category_repo,
            collection_repo,
            content_manager_repo,
            interaction_repo,
            publication_repo,
            reader_repo,
            statistics_repo,
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

    pub fn statistics_repo(&self) -> &SRepo {
        &self.statistics_repo
    }
}
