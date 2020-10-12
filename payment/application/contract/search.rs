use std::str::FromStr;

use chrono::DateTime;
use serde::{Deserialize, Serialize};

use common::error::Error;
use common::request::{Include, PaginationParams};
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use publishing::application::dtos::PublicationDto;
use publishing::domain::publication::{PublicationId, PublicationRepository};

use crate::application::dtos::ContractDto;
use crate::domain::contract::ContractRepository;

#[derive(Deserialize)]
pub struct SearchCommand {
    pub publication_id: Option<String>,
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    contracts: Vec<ContractDto>,
}

pub struct Search<'a> {
    contract_repo: &'a dyn ContractRepository,
    publication_repo: &'a dyn PublicationRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Search<'a> {
    pub fn new(
        contract_repo: &'a dyn ContractRepository,
        publication_repo: &'a dyn PublicationRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Search {
            contract_repo,
            publication_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        auth_id: String,
        cmd: SearchCommand,
        include: Include,
        pagination: PaginationParams,
    ) -> Result<SearchResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_content_manager() {
            return Err(Error::unauthorized());
        }

        let contracts = self
            .contract_repo
            .search(
                cmd.publication_id
                    .map(PublicationId::new)
                    .transpose()?
                    .as_ref(),
                cmd.status.as_ref(),
                cmd.date_from
                    .map(|d| DateTime::from_str(&d))
                    .transpose()
                    .map_err(|err| Error::bad_format("date_from").wrap_raw(err))?
                    .as_ref(),
                cmd.date_to
                    .map(|d| DateTime::from_str(&d))
                    .transpose()
                    .map_err(|err| Error::bad_format("date_to").wrap_raw(err))?
                    .as_ref(),
                pagination.offset,
                pagination.limit,
            )
            .await?;

        let mut contract_dtos = Vec::new();

        for contract in contracts.iter() {
            let mut contract_dto = ContractDto::from(contract);

            if include.has("publication") {
                let publication = self
                    .publication_repo
                    .find_by_id(contract.publication_id())
                    .await?;
                contract_dto = contract_dto.publication(PublicationDto::from(&publication));
            }

            contract_dtos.push(contract_dto);
        }

        Ok(SearchResponse {
            contracts: contract_dtos,
        })
    }
}
