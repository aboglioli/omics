use std::str::FromStr;

use chrono::DateTime;
use serde::{Deserialize, Serialize};

use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;

use identity::domain::user::{UserId, UserRepository};

use crate::domain::contract::{ContractRepository, ContractService};

#[derive(Deserialize)]
pub struct GenerateStatisticsCommand {
    from: String,
    to: String,
}

#[derive(Serialize)]
pub struct GenerateStatisticsResponse {
    updated_contracts: u64,
}

pub struct GenerateStatistics<'a> {
    event_pub: &'a dyn EventPublisher,

    contract_repo: &'a dyn ContractRepository,
    user_repo: &'a dyn UserRepository,

    contract_serv: &'a ContractService,
}

impl<'a> GenerateStatistics<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        contract_repo: &'a dyn ContractRepository,
        user_repo: &'a dyn UserRepository,
        contract_serv: &'a ContractService,
    ) -> Self {
        GenerateStatistics {
            event_pub,
            contract_repo,
            user_repo,
            contract_serv,
        }
    }

    pub async fn exec(
        &self,
        auth_id: String,
        cmd: GenerateStatisticsCommand,
    ) -> Result<GenerateStatisticsResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

        let from = DateTime::from_str(&cmd.from)
            .map_err(|err| Error::bad_format("date_from").wrap_raw(err))?;
        let to = DateTime::from_str(&cmd.to)
            .map_err(|err| Error::bad_format("date_to").wrap_raw(err))?;

        let contracts = self.contract_serv.calculate_summaries(from, to).await?;

        let total_contracts = contracts.len();

        for mut contract in contracts.into_iter() {
            self.contract_repo.save(&mut contract).await?;
            self.event_pub
                .publish_all(contract.events().to_vec()?)
                .await?;
        }

        Ok(GenerateStatisticsResponse {
            updated_contracts: total_contracts as u64,
        })
    }
}
