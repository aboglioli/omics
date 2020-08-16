use crate::domain::contract::Contract;

pub struct ContractDto {
    pub id: String,
    pub publication_id: String,
}

impl ContractDto {
    pub fn new(contract: &Contract) -> Self {
        ContractDto {
            id: contract.base().id().value().to_owned(),
            publication_id: contract.publication_id().value().to_owned(),
        }
    }
}
