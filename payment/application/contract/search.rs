use std::str::FromStr;

use chrono::DateTime;
use serde::Deserialize;

use common::error::Error;
use common::request::{Include, PaginationParams, PaginationResponse};
use common::result::Result;
use identity::UserIdAndRole;
use publishing::application::dtos::PublicationDto;
use publishing::domain::publication::{PublicationId, PublicationRepository};

use crate::application::dtos::ContractDto;
use crate::domain::contract::{ContractOrderBy, ContractRepository, Status};

#[derive(Deserialize)]
pub struct SearchCommand {
    pub publication_id: Option<String>,
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

pub struct Search<'a> {
    contract_repo: &'a dyn ContractRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> Search<'a> {
    pub fn new(
        contract_repo: &'a dyn ContractRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        Search {
            contract_repo,
            publication_repo,
        }
    }

    pub async fn exec(
        &self,
        (_auth_id, auth_role): UserIdAndRole,
        cmd: SearchCommand,
        include: Include,
        pagination: PaginationParams,
    ) -> Result<PaginationResponse<ContractDto>> {
        if !auth_role.can("get_any_contract") {
            return Err(Error::unauthorized());
        }

        let pagination_contracts = self
            .contract_repo
            .search(
                cmd.publication_id
                    .map(PublicationId::new)
                    .transpose()?
                    .as_ref(),
                cmd.status
                    .map(|s| Status::from_str(&s))
                    .transpose()?
                    .as_ref(),
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
                pagination.offset(),
                pagination.limit(),
                pagination
                    .order_by()
                    .map(|o| ContractOrderBy::from_str(&o))
                    .transpose()?
                    .as_ref(),
            )
            .await?;

        let mut res = PaginationResponse::from(&pagination_contracts);
        // let mut res = PaginationResponse::new(
        //     pagination_contracts.offset(),
        //     pagination_contracts.limit(),
        //     pagination_contracts.total(),
        //     pagination_contracts.matching_criteria(),
        // );

        for contract in pagination_contracts.into_items().into_iter() {
            let mut contract_dto = ContractDto::from(&contract);

            if include.has("publication") {
                let publication = self
                    .publication_repo
                    .find_by_id(contract.publication_id())
                    .await?;
                contract_dto = contract_dto.publication(PublicationDto::from(&publication));
            }

            res.add_item(contract_dto);
        }

        Ok(res)
    }
}
