use serde::Serialize;

use common::error::Error;
use common::result::Result;
use identity::UserIdAndRole;
use publishing::domain::publication::{PublicationId, PublicationRepository};

use crate::domain::contract::ContractService;

#[derive(Serialize)]
pub struct CanRequestResponse {
    can_request: bool,
}

pub struct CanRequest<'a> {
    publication_repo: &'a dyn PublicationRepository,

    contract_serv: &'a ContractService,
}

impl<'a> CanRequest<'a> {
    pub fn new(
        publication_repo: &'a dyn PublicationRepository,
        contract_serv: &'a ContractService,
    ) -> Self {
        CanRequest {
            publication_repo,
            contract_serv,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        publication_id: String,
    ) -> Result<CanRequestResponse> {
        let publication_id = PublicationId::new(publication_id)?;
        let publication = self.publication_repo.find_by_id(&publication_id).await?;

        if publication.author_id() != &auth_id || !auth_role.can("request_contract") {
            return Err(Error::unauthorized());
        }

        match self.contract_serv.can_request(&publication_id).await {
            Ok(()) => Ok(CanRequestResponse { can_request: true }),
            Err(_) => Ok(CanRequestResponse { can_request: false }),
        }
    }
}
