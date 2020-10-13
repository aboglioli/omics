use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};

use common::error::Error;
use common::result::Result;
use publishing::domain::publication::{
    PublicationId, PublicationRepository, StatisticsService, Status as PublicationStatus,
};

use crate::domain::contract::{Contract, ContractRepository, Status, Summary};
use crate::domain::subscription::SubscriptionRepository;

pub struct ContractService {
    contract_repo: Arc<dyn ContractRepository>,
    publication_repo: Arc<dyn PublicationRepository>,
    subscription_repo: Arc<dyn SubscriptionRepository>,

    statistics_serv: Arc<StatisticsService>,
}

impl ContractService {
    pub fn new(
        contract_repo: Arc<dyn ContractRepository>,
        publication_repo: Arc<dyn PublicationRepository>,
        subscription_repo: Arc<dyn SubscriptionRepository>,
        statistics_serv: Arc<StatisticsService>,
    ) -> Self {
        ContractService {
            contract_repo,
            publication_repo,
            subscription_repo,
            statistics_serv,
        }
    }

    pub async fn can_request(&self, publication_id: &PublicationId) -> Result<()> {
        if let Ok(contract) = self
            .contract_repo
            .find_by_publication_id(&publication_id)
            .await
        {
            if matches!(
                contract.status_history().current(),
                Status::Requested | Status::Approved { .. }
            ) {
                return Err(Error::new("contract", "already_exists"));
            }
        }

        let publication = self.publication_repo.find_by_id(publication_id).await?;
        if !matches!(publication.status_history().current(), PublicationStatus::Published { .. }) {
            return Err(Error::new("publication", "not_published"));
        }

        let now = Utc::now();
        let last_month = now - Duration::days(30);

        let total_statistics = self
            .statistics_serv
            .get_history(None, None, Some(&last_month), Some(&now))
            .await?;
        let publication_statistics = self
            .statistics_serv
            .get_history(None, Some(publication_id), Some(&last_month), Some(&now))
            .await?;
        let p = publication_statistics.views() as f64 / total_statistics.views() as f64;

        if p >= 0.01 {
            return Ok(());
        }

        Err(Error::new("statistics", "publication_has_low_views"))
    }

    pub async fn calculate_summaries(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<Contract>> {
        let contracts = self
            .contract_repo
            .search(None, Some(&"approved".to_owned()), None, None, None, None)
            .await?;
        let subscriptions = self.subscription_repo.search(None, None, None).await?;

        let mut subscription_total: f64 = 0.0;
        for subscription in subscriptions.iter() {
            for payment in subscription.payments().iter() {
                if payment.datetime() >= &from && payment.datetime() <= &to {
                    subscription_total += payment.amount().value();
                }
            }
        }
        subscription_total = subscription_total * 0.3;

        if subscription_total == 0.0 {
            return Err(Error::new("subscription_total", "zero"));
        }

        let mut total_views = 0;
        let mut contract_statistics = Vec::new();
        for contract in contracts.into_iter() {
            let statistics = self
                .statistics_serv
                .get_history(
                    None,
                    Some(contract.publication_id()),
                    Some(&from),
                    Some(&to),
                )
                .await?;

            total_views += statistics.views();
            contract_statistics.push((contract, statistics));
        }

        let mut contracts = Vec::new();
        for (mut contract, statistics) in contract_statistics.into_iter() {
            let views = statistics.views();
            contract.add_summary(Summary::new(
                statistics,
                subscription_total,
                (views as f64 / total_views as f64) * subscription_total,
                from.clone(),
                to.clone(),
            )?)?;
            contracts.push(contract);
        }

        Ok(contracts)
    }

    pub async fn calculate_summaries_for_publication(
        &self,
        publication_id: &PublicationId,
    ) -> Result<Contract> {
        let mut contract = self
            .contract_repo
            .find_by_publication_id(publication_id)
            .await?;

        if !matches!(contract.status_history().current(), Status::Approved{ .. }) {
            return Err(Error::new("contract", "not_approved"));
        }

        let date_from = if let Some(last_summary) = contract.summaries().last() {
            last_summary.to().clone()
        } else {
            contract.status_history().current_item().datetime().clone()
        };

        // Only update contracts older than 10 days
        if date_from + Duration::days(10) > Utc::now() {
            return Ok(contract);
        }

        let subscriptions = self.subscription_repo.search(None, None, None).await?;
        let mut subscription_total: f64 = 0.0;
        for subscription in subscriptions.iter() {
            for payment in subscription.payments().iter() {
                if payment.datetime() >= &date_from {
                    subscription_total += payment.amount().value();
                }
            }
        }
        subscription_total = subscription_total * 0.3;

        if subscription_total == 0.0 {
            return Err(Error::new("subscription_total", "zero"));
        }

        let statistics = self
            .statistics_serv
            .get_history(
                None,
                Some(contract.publication_id()),
                Some(&date_from),
                None,
            )
            .await?;

        let contracts = self
            .contract_repo
            .search(None, Some(&"approved".to_owned()), None, None, None, None)
            .await?;
        let mut total_views = 0;
        for contract in contracts.into_iter() {
            let statistics = self
                .statistics_serv
                .get_history(
                    None,
                    Some(contract.publication_id()),
                    Some(&date_from),
                    None,
                )
                .await?;

            total_views += statistics.views();
        }

        let views = statistics.views();
        contract.add_summary(Summary::new(
            statistics,
            subscription_total,
            (views as f64 / total_views as f64) * subscription_total,
            date_from.clone(),
            Utc::now(),
        )?)?;

        Ok(contract)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    use async_trait::async_trait;

    use common::model::{AggregateRoot, StatusHistory, StatusItem};
    use identity::domain::user::UserId;
    use publishing::domain::author::AuthorId;
    use publishing::domain::collection::CollectionId;
    use publishing::domain::interaction::{
        CollectionFavorite, Comment, Follow, InteractionRepository, Like, PublicationFavorite,
        ReaderPublicationId, Reading, Review, Stars, View,
    };
    use publishing::domain::publication::{PublicationId, Statistics};
    use publishing::domain::reader::ReaderId;
    use publishing::infrastructure::persistence::inmem::InMemPublicationRepository;
    use publishing::mocks as publishing_mocks;

    use crate::domain::contract::{ContractId, Status as ContractStatus};
    use crate::domain::payment::{Amount, Kind, Payment};
    use crate::domain::plan::{Plan, PlanId, Price};
    use crate::domain::subscription::{
        Status as SubscriptionStatus, Subscription, SubscriptionId, SubscriptionPlan,
    };

    struct FakeContractRepository;

    #[async_trait]
    impl ContractRepository for FakeContractRepository {
        async fn find_by_id(&self, _id: &ContractId) -> Result<Contract> {
            unimplemented!()
        }

        async fn find_by_publication_id(&self, id: &PublicationId) -> Result<Contract> {
            let contracts = self
                .search(None, Some(&"approved".to_owned()), None, None, None, None)
                .await?;
            for contract in contracts.into_iter() {
                if contract.publication_id() == id {
                    return Ok(contract);
                }
            }
            Err(Error::not_found("contract"))
        }

        async fn search(
            &self,
            _publication_id: Option<&PublicationId>,
            status: Option<&String>,
            _from: Option<&DateTime<Utc>>,
            _to: Option<&DateTime<Utc>>,
            _offset: Option<usize>,
            _limit: Option<usize>,
        ) -> Result<Vec<Contract>> {
            assert_eq!(status.unwrap(), "approved");

            let statistics = Statistics::new(5000, 5000, 5000, 5000, 5000, 4.5)?;

            let mut publication_1 = publishing_mocks::publication(
                "#publication01",
                "#user01",
                "Publication 1",
                "adventure",
                vec!["Tag 1"],
                "cover.jpg",
                2,
                true,
                true,
                false,
            );
            publication_1.set_statistics(statistics.clone())?;

            let mut publication_2 = publishing_mocks::publication(
                "#publication02",
                "#user02",
                "Publication 2",
                "adventure",
                vec!["Tag 1"],
                "cover.jpg",
                2,
                true,
                true,
                false,
            );
            publication_2.set_statistics(statistics.clone())?;

            let mut publication_3 = publishing_mocks::publication(
                "#publication03",
                "#user03",
                "Publication 3",
                "adventure",
                vec!["Tag 1"],
                "cover.jpg",
                2,
                true,
                true,
                false,
            );
            publication_3.set_statistics(statistics)?;

            Ok(vec![
                Contract::build(
                    AggregateRoot::new(ContractId::new("#contract01")?),
                    PublicationId::new("#publication01")?,
                    Vec::new(),
                    Vec::new(),
                    StatusHistory::build(vec![
                        StatusItem::new(ContractStatus::init()),
                        StatusItem::new(ContractStatus::Approved {
                            admin_id: UserId::new("#admin")?,
                        }),
                    ]),
                ),
                Contract::build(
                    AggregateRoot::new(ContractId::new("#contract02")?),
                    PublicationId::new("#publication02")?,
                    Vec::new(),
                    Vec::new(),
                    StatusHistory::build(vec![
                        StatusItem::new(ContractStatus::init()),
                        StatusItem::new(ContractStatus::Approved {
                            admin_id: UserId::new("#admin")?,
                        }),
                    ]),
                ),
                Contract::build(
                    AggregateRoot::new(ContractId::new("#contract03")?),
                    PublicationId::new("#publication03")?,
                    Vec::new(),
                    Vec::new(),
                    StatusHistory::build(vec![
                        StatusItem::new(ContractStatus::init()),
                        StatusItem::new(ContractStatus::Approved {
                            admin_id: UserId::new("#admin")?,
                        }),
                    ]),
                ),
            ])
        }

        async fn save(&self, _contract: &mut Contract) -> Result<()> {
            unimplemented!()
        }

        async fn delete(&self, _id: &ContractId) -> Result<()> {
            unimplemented!()
        }
    }

    struct FakeSubscriptionRepository;

    #[async_trait]
    impl SubscriptionRepository for FakeSubscriptionRepository {
        async fn find_by_id(&self, _id: &SubscriptionId) -> Result<Subscription> {
            unimplemented!()
        }

        async fn find_by_user_id(&self, _id: &UserId) -> Result<Subscription> {
            unimplemented!()
        }

        // Subscription 1: $225 (in date range)
        // Subscription 2: $225 (in date range)
        async fn search(
            &self,
            _user_id: Option<&UserId>,
            _plan_id: Option<&PlanId>,
            _status: Option<&String>,
        ) -> Result<Vec<Subscription>> {
            let plan = Plan::new(PlanId::new("basic")?, "Basic", "Basic", Price::new(75.0)?)?;

            let subscription_1 = Subscription::build(
                AggregateRoot::new(SubscriptionId::new("#subscription01")?),
                UserId::new("#user01")?,
                SubscriptionPlan::new(plan.clone())?,
                vec![
                    Payment::build(
                        Kind::Income,
                        Amount::new(75.0)?,
                        DateTime::from_str("2020-04-15T15:30:00Z").unwrap(),
                    ),
                    Payment::build(
                        Kind::Income,
                        Amount::new(75.0)?,
                        DateTime::from_str("2020-05-01T14:30:00Z").unwrap(),
                    ),
                    Payment::build(
                        Kind::Income,
                        Amount::new(75.0)?,
                        DateTime::from_str("2020-05-15T14:30:00Z").unwrap(),
                    ),
                    Payment::build(
                        Kind::Income,
                        Amount::new(75.0)?,
                        DateTime::from_str("2020-05-30T14:30:00Z").unwrap(),
                    ),
                    Payment::build(
                        Kind::Income,
                        Amount::new(75.0)?,
                        DateTime::from_str("2020-06-01T15:30:00Z").unwrap(),
                    ),
                ],
                StatusHistory::build(vec![
                    StatusItem::new(SubscriptionStatus::init()),
                    StatusItem::new(SubscriptionStatus::Active),
                ]),
            );

            let subscription_2 = Subscription::build(
                AggregateRoot::new(SubscriptionId::new("#subscription02")?),
                UserId::new("#user02")?,
                SubscriptionPlan::new(plan.clone())?,
                vec![
                    Payment::build(
                        Kind::Income,
                        Amount::new(75.0)?,
                        DateTime::from_str("2020-05-01T14:30:00Z").unwrap(),
                    ),
                    Payment::build(
                        Kind::Income,
                        Amount::new(75.0)?,
                        DateTime::from_str("2020-05-15T14:30:00Z").unwrap(),
                    ),
                    Payment::build(
                        Kind::Income,
                        Amount::new(75.0)?,
                        DateTime::from_str("2020-05-30T14:30:00Z").unwrap(),
                    ),
                    Payment::build(
                        Kind::Income,
                        Amount::new(75.0)?,
                        DateTime::from_str("2020-06-01T14:30:00Z").unwrap(),
                    ),
                ],
                StatusHistory::build(vec![
                    StatusItem::new(SubscriptionStatus::init()),
                    StatusItem::new(SubscriptionStatus::Active),
                ]),
            );

            Ok(vec![subscription_1, subscription_2])
        }

        async fn save(&self, _subscription: &mut Subscription) -> Result<()> {
            unimplemented!()
        }

        async fn delete(&self, _id: &SubscriptionId) -> Result<()> {
            unimplemented!()
        }
    }

    struct FakeInteractionRepository;

    #[async_trait]
    impl InteractionRepository for FakeInteractionRepository {
        // Publication 1: 2 views
        // Publication 2: 1 view
        // Publication 3: 0 views
        async fn find_views(
            &self,
            _reader_id: Option<&ReaderId>,
            publication_id: Option<&PublicationId>,
            _from: Option<&DateTime<Utc>>,
            _to: Option<&DateTime<Utc>>,
        ) -> Result<Vec<View>> {
            match publication_id.unwrap().value() {
                "#publication01" => Ok(vec![
                    View::new(
                        ReaderPublicationId::new(
                            ReaderId::new("#user02")?,
                            PublicationId::new("#publication01")?,
                        )?,
                        true,
                    )?,
                    View::new(
                        ReaderPublicationId::new(
                            ReaderId::new("#user03")?,
                            PublicationId::new("#publication01")?,
                        )?,
                        true,
                    )?,
                ]),
                "#publication02" => Ok(vec![View::new(
                    ReaderPublicationId::new(
                        ReaderId::new("#user01")?,
                        PublicationId::new("#publication02")?,
                    )?,
                    true,
                )?]),
                "#publication03" => Ok(Vec::new()),
                id => Err(Error::not_found("publication").set_message(id)),
            }
        }
        async fn find_readings(
            &self,
            _reader_id: Option<&ReaderId>,
            publication_id: Option<&PublicationId>,
            _from: Option<&DateTime<Utc>>,
            _to: Option<&DateTime<Utc>>,
        ) -> Result<Vec<Reading>> {
            match publication_id.unwrap().value() {
                "#publication01" => Ok(vec![
                    Reading::new(ReaderPublicationId::new(
                        ReaderId::new("#user02")?,
                        PublicationId::new("#publication01")?,
                    )?)?,
                    Reading::new(ReaderPublicationId::new(
                        ReaderId::new("#user03")?,
                        PublicationId::new("#publication01")?,
                    )?)?,
                ]),
                "#publication02" => Ok(vec![Reading::new(ReaderPublicationId::new(
                    ReaderId::new("#user01")?,
                    PublicationId::new("#publication02")?,
                )?)?]),
                "#publication03" => Ok(Vec::new()),
                id => Err(Error::not_found("publication").set_message(id)),
            }
        }
        async fn find_likes(
            &self,
            _reader_id: Option<&ReaderId>,
            publication_id: Option<&PublicationId>,
            _from: Option<&DateTime<Utc>>,
            _to: Option<&DateTime<Utc>>,
        ) -> Result<Vec<Like>> {
            match publication_id.unwrap().value() {
                "#publication01" => Ok(vec![
                    Like::new(ReaderPublicationId::new(
                        ReaderId::new("#user02")?,
                        PublicationId::new("#publication01")?,
                    )?)?,
                    Like::new(ReaderPublicationId::new(
                        ReaderId::new("#user03")?,
                        PublicationId::new("#publication01")?,
                    )?)?,
                ]),
                "#publication02" => Ok(vec![Like::new(ReaderPublicationId::new(
                    ReaderId::new("#user01")?,
                    PublicationId::new("#publication02")?,
                )?)?]),
                "#publication03" => Ok(Vec::new()),
                id => Err(Error::not_found("publication").set_message(id)),
            }
        }
        async fn find_reviews(
            &self,
            _reader_id: Option<&ReaderId>,
            publication_id: Option<&PublicationId>,
            _from: Option<&DateTime<Utc>>,
            _to: Option<&DateTime<Utc>>,
        ) -> Result<Vec<Review>> {
            match publication_id.unwrap().value() {
                "#publication01" => Ok(vec![
                    Review::new(
                        ReaderPublicationId::new(
                            ReaderId::new("#user02")?,
                            PublicationId::new("#publication01")?,
                        )?,
                        Stars::new(4)?,
                        Comment::new("Comment...")?,
                    )?,
                    Review::new(
                        ReaderPublicationId::new(
                            ReaderId::new("#user03")?,
                            PublicationId::new("#publication01")?,
                        )?,
                        Stars::new(4)?,
                        Comment::new("Comment...")?,
                    )?,
                ]),
                "#publication02" => Ok(vec![Review::new(
                    ReaderPublicationId::new(
                        ReaderId::new("#user01")?,
                        PublicationId::new("#publication02")?,
                    )?,
                    Stars::new(4)?,
                    Comment::new("Comment...")?,
                )?]),
                "#publication03" => Ok(Vec::new()),
                id => Err(Error::not_found("publication").set_message(id)),
            }
        }
        async fn find_publication_favorites(
            &self,
            _reader_id: Option<&ReaderId>,
            _publication_id: Option<&PublicationId>,
            _from: Option<&DateTime<Utc>>,
            _to: Option<&DateTime<Utc>>,
        ) -> Result<Vec<PublicationFavorite>> {
            unimplemented!()
        }
        async fn find_collection_favorites(
            &self,
            _reader_id: Option<&ReaderId>,
            _collection_id: Option<&CollectionId>,
            _from: Option<&DateTime<Utc>>,
            _to: Option<&DateTime<Utc>>,
        ) -> Result<Vec<CollectionFavorite>> {
            unimplemented!()
        }
        async fn find_follows(
            &self,
            _reader_id: Option<&ReaderId>,
            _author_id: Option<&AuthorId>,
            _from: Option<&DateTime<Utc>>,
            _to: Option<&DateTime<Utc>>,
        ) -> Result<Vec<Follow>> {
            unimplemented!()
        }

        async fn save_view(&self, _view: &mut View) -> Result<()> {
            unimplemented!()
        }
        async fn save_reading(&self, _reading: &mut Reading) -> Result<()> {
            unimplemented!()
        }
        async fn save_like(&self, _like: &mut Like) -> Result<()> {
            unimplemented!()
        }
        async fn save_review(&self, _review: &mut Review) -> Result<()> {
            unimplemented!()
        }
        async fn save_publication_favorite(
            &self,
            _favorite: &mut PublicationFavorite,
        ) -> Result<()> {
            unimplemented!()
        }
        async fn save_collection_favorite(&self, _favorite: &mut CollectionFavorite) -> Result<()> {
            unimplemented!()
        }
        async fn save_follow(&self, _follow: &mut Follow) -> Result<()> {
            unimplemented!()
        }

        async fn delete_like(
            &self,
            _reader_id: &ReaderId,
            _publication_id: &PublicationId,
        ) -> Result<()> {
            unimplemented!()
        }
        async fn delete_review(
            &self,
            _reader_id: &ReaderId,
            _publication_id: &PublicationId,
        ) -> Result<()> {
            unimplemented!()
        }
        async fn delete_publication_favorite(
            &self,
            _reader_id: &ReaderId,
            _publication_id: &PublicationId,
        ) -> Result<()> {
            unimplemented!()
        }
        async fn delete_collection_favorite(
            &self,
            _reader_id: &ReaderId,
            _collection_id: &CollectionId,
        ) -> Result<()> {
            unimplemented!()
        }
        async fn delete_follow(&self, _reader_id: &ReaderId, _author_id: &AuthorId) -> Result<()> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn calculate_summaries() {
        let contract_serv = ContractService::new(
            Arc::new(FakeContractRepository),
            Arc::new(InMemPublicationRepository::new()),
            Arc::new(FakeSubscriptionRepository),
            Arc::new(StatisticsService::new(Arc::new(FakeInteractionRepository))),
        );

        // Subscription amount: $450
        // Total views: 3 views
        let contracts = contract_serv
            .calculate_summaries(
                DateTime::from_str("2020-05-01T14:30:00Z").unwrap(),
                DateTime::from_str("2020-05-31T14:30:00Z").unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(contracts.len(), 3);

        let summaries = contracts[0].summaries();
        assert_eq!(summaries.len(), 1);
        assert!(!summaries[0].is_paid());
        assert_eq!(summaries[0].total(), 450.0 * 0.3);
        assert_eq!(summaries[0].amount(), 2.0 / 3.0 * 450.0 * 0.3);

        let summaries = contracts[1].summaries();
        assert_eq!(summaries.len(), 1);
        assert!(!summaries[0].is_paid());
        assert_eq!(summaries[0].total(), 450.0 * 0.3);
        assert_eq!(summaries[0].amount(), 1.0 / 3.0 * 450.0 * 0.3);

        let summaries = contracts[2].summaries();
        assert_eq!(summaries.len(), 1);
        assert!(!summaries[0].is_paid());
        assert_eq!(summaries[0].total(), 450.0 * 0.3);
        assert_eq!(summaries[0].amount(), 0.0 / 3.0 * 450.0 * 0.3);

        // TODO: consider dates in status
        // let contract = contract_serv
        //     .calculate_summaries_for_publication(&PublicationId::new("#publication01").unwrap())
        //     .await
        //     .unwrap();
        // let summaries = contract.summaries();
        // assert_eq!(summaries.len(), 1);
        // assert!(!summaries[0].is_paid());
        // assert_eq!(summaries[0].total(), 450.0);
        // assert_eq!(summaries[0].amount(), 2.0 / 3.0 * 450.0);
    }
}
