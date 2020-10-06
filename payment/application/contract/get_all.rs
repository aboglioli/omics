use serde::Serialize;

use common::error::Error;
use common::request::Include;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use publishing::application::dtos::PublicationDto;
use publishing::domain::publication::{PublicationRepository};

use crate::application::dtos::ContractDto;
use crate::domain::contract::ContractRepository;

#[derive(Serialize)]
pub struct GetAllResponse {
    contracts: Vec<ContractDto>,
}

pub struct GetAll<'a> {
    contract_repo: &'a dyn ContractRepository,
    publication_repo: &'a dyn PublicationRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> GetAll<'a> {
    pub fn new(
        contract_repo: &'a dyn ContractRepository,
        publication_repo: &'a dyn PublicationRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        GetAll {
            contract_repo,
            publication_repo,
            user_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, include: Include) -> Result<GetAllResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.is_admin() {
            return Err(Error::unauthorized());
        }

        let contracts = self.contract_repo.search(None, None).await?;

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

        Ok(GetAllResponse {
            contracts: contract_dtos,
        })
    }
}
