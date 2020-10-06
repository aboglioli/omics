use std::sync::Arc;

use async_trait::async_trait;

use common::container::Container;
use common::event::{EventPublisher, EventSubscriber};
use common::result::Result;
use identity::domain::user::UserRepository;

use crate::application::publication::ContractHandler;
use crate::application::reader::SubscriptionHandler;
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::CollectionRepository;
use crate::domain::interaction::InteractionRepository;
use crate::domain::publication::{PublicationRepository, StatisticsService};
use crate::domain::reader::ReaderRepository;

pub struct PublishingContainer<EPub> {
    event_pub: Arc<EPub>,

    author_repo: Arc<dyn AuthorRepository>,
    category_repo: Arc<dyn CategoryRepository>,
    collection_repo: Arc<dyn CollectionRepository>,
    interaction_repo: Arc<dyn InteractionRepository>,
    publication_repo: Arc<dyn PublicationRepository>,
    reader_repo: Arc<dyn ReaderRepository>,
    user_repo: Arc<dyn UserRepository>,

    statistics_serv: Arc<StatisticsService>,
}

impl<EPub> PublishingContainer<EPub>
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
    ) -> Self {
        let statistics_serv = Arc::new(StatisticsService::new(interaction_repo.clone()));

        PublishingContainer {
            event_pub,

            author_repo,
            category_repo,
            collection_repo,
            interaction_repo,
            publication_repo,
            reader_repo,
            user_repo,

            statistics_serv,
        }
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
}

#[async_trait]
impl<EPub> Container for PublishingContainer<EPub>
where
    EPub: Sync + Send,
{
    async fn subscribe<ES>(&self, event_sub: &ES) -> Result<()>
    where
        ES: EventSubscriber + Sync + Send,
    {
        // let author_from_user_handler = AuthorFromUserHandler::new(self.author_repo.clone());
        // event_sub
        //     .subscribe(Box::new(author_from_user_handler))
        //     .await?;
        //
        // let reader_from_user_handler = ReaderFromUserHandler::new(self.reader_repo.clone());
        // event_sub
        //     .subscribe(Box::new(reader_from_user_handler))
        //     .await?;
        //
        // let reader_handler =
        //     InteractionHandler::new(self.reader_repo.clone(), self.publication_repo.clone());
        // event_sub.subscribe(Box::new(reader_handler)).await?;

        let subscription_handler = SubscriptionHandler::new(self.reader_repo.clone());
        event_sub.subscribe(Box::new(subscription_handler)).await?;

        let contract_handler = ContractHandler::new(self.publication_repo.clone());
        event_sub.subscribe(Box::new(contract_handler)).await?;

        Ok(())
    }
}
