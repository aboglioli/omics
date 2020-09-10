use std::sync::Arc;

use common::event::{EventPublisher, EventSubscriber};
use common::result::Result;

use crate::application::handler::UserHandler;
use crate::application::reader::InteractionHandler;
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::CollectionRepository;
use crate::domain::interaction::{InteractionRepository, InteractionService};
use crate::domain::publication::{PublicationRepository, StatisticsService};
use crate::domain::reader::ReaderRepository;
use crate::domain::user::{UserRepository, UserService};

pub struct Container<EPub> {
    event_pub: Arc<EPub>,

    author_repo: Arc<dyn AuthorRepository>,
    category_repo: Arc<dyn CategoryRepository>,
    collection_repo: Arc<dyn CollectionRepository>,
    interaction_repo: Arc<dyn InteractionRepository>,
    publication_repo: Arc<dyn PublicationRepository>,
    reader_repo: Arc<dyn ReaderRepository>,
    user_repo: Arc<dyn UserRepository>,

    statistics_serv: Arc<StatisticsService>,
    interaction_serv: Arc<InteractionService>,

    user_serv: Arc<dyn UserService>,
}

impl<EPub> Container<EPub>
where
    EPub: EventPublisher,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        event_pub: Arc<EPub>,
        author_repo: Arc<dyn AuthorRepository>,
        category_repo: Arc<dyn CategoryRepository>,
        collection_repo: Arc<dyn CollectionRepository>,
        interaction_repo: Arc<dyn InteractionRepository>,
        publication_repo: Arc<dyn PublicationRepository>,
        reader_repo: Arc<dyn ReaderRepository>,
        user_repo: Arc<dyn UserRepository>,
        user_serv: Arc<dyn UserService>,
    ) -> Self {
        let statistics_serv = Arc::new(StatisticsService::new(interaction_repo.clone()));
        let interaction_serv = Arc::new(InteractionService::new(interaction_repo.clone()));

        Container {
            event_pub,

            author_repo,
            category_repo,
            collection_repo,
            interaction_repo,
            publication_repo,
            reader_repo,
            user_repo,

            statistics_serv,
            interaction_serv,

            user_serv,
        }
    }

    pub async fn subscribe<ES>(&self, event_sub: &ES) -> Result<()>
    where
        ES: EventSubscriber,
    {
        let user_handler = UserHandler::new(
            self.author_repo.clone(),
            self.reader_repo.clone(),
            self.user_repo.clone(),
            self.user_serv.clone(),
        );
        event_sub.subscribe(Box::new(user_handler)).await?;

        let reader_handler =
            InteractionHandler::new(self.reader_repo.clone(), self.publication_repo.clone());
        event_sub.subscribe(Box::new(reader_handler)).await?;

        Ok(())
    }

    pub fn event_pub(&self) -> &EPub {
        &self.event_pub
    }

    pub fn author_repo(&self) -> &dyn AuthorRepository {
        self.author_repo.as_ref()
    }

    pub fn category_repo(&self) -> &dyn CategoryRepository {
        self.category_repo.as_ref()
    }

    pub fn collection_repo(&self) -> &dyn CollectionRepository {
        self.collection_repo.as_ref()
    }

    pub fn interaction_repo(&self) -> &dyn InteractionRepository {
        self.interaction_repo.as_ref()
    }

    pub fn publication_repo(&self) -> &dyn PublicationRepository {
        self.publication_repo.as_ref()
    }

    pub fn reader_repo(&self) -> &dyn ReaderRepository {
        self.reader_repo.as_ref()
    }

    pub fn user_repo(&self) -> &dyn UserRepository {
        self.user_repo.as_ref()
    }

    // Concrete services
    pub fn statistics_serv(&self) -> &StatisticsService {
        &self.statistics_serv
    }

    pub fn interaction_serv(&self) -> &InteractionService {
        &self.interaction_serv
    }

    // Abstract services
    pub fn user_serv(&self) -> &dyn UserService {
        self.user_serv.as_ref()
    }
}
