use std::sync::Arc;

use chrono::{DateTime, Utc};

use common::result::Result;
use publishing::domain::publication::StatisticsService;

use crate::domain::contract::{ContractRepository, Summary};
use crate::domain::subscription::SubscriptionRepository;

pub struct ContractService {
    contract_repo: Arc<dyn ContractRepository>,
    subscription_repo: Arc<dyn SubscriptionRepository>,

    statistics_serv: Arc<StatisticsService>,
}

impl ContractService {
    pub fn new(
        contract_repo: Arc<dyn ContractRepository>,
        subscription_repo: Arc<dyn SubscriptionRepository>,
        statistics_serv: Arc<StatisticsService>,
    ) -> Self {
        ContractService {
            contract_repo,
            subscription_repo,
            statistics_serv,
        }
    }

    pub async fn calculate_summaries(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<()> {
        let contracts = self.contract_repo.search(None, Some(&"approved".to_owned())).await?;
        let subscriptions = self.subscription_repo.search(
            None,
            None,
            None,
        ).await?;

        let mut subscription_total: f64 = 0.0;
        for subscription in subscriptions.iter() {
            for payment in subscription.payments().iter() {
                if payment.datetime() >= &from && payment.datetime() <= &to {
                    subscription_total += payment.amount().value();
                }
            }
        }

        let mut total_views = 0;
        let mut contract_statistics = Vec::new();
        for contract in contracts.into_iter() {
            let statistics = self.statistics_serv.get_history(
                None,
                Some(contract.publication_id()),
                Some(&from),
                Some(&to),
            ).await?;

            total_views += statistics.views();
            contract_statistics.push((contract, statistics));
        }

        for (mut contract, statistics) in contract_statistics.into_iter() {
            let views = statistics.views();
            contract.add_summary(
                Summary::new(
                    statistics,
                    subscription_total,
                    (views as f64 / total_views as f64) * subscription_total,
                    from.clone(),
                    to.clone(),
                )?
            )?;

            self.contract_repo.save(&mut contract).await?;
        }

        Ok(())
    }
}
