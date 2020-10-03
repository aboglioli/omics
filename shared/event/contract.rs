use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub enum ContractEvent {
    Requested {
        id: String,
        publication_id: String,
        author_id: String,
    },
    Approved {
        id: String,
        publication_id: String,
        admin_id: String,
    },
    Rejected {
        id: String,
        publication_id: String,
        admin_id: String,
    },
    Cancelled {
        id: String,
        publication_id: String,
    },
}

impl ToString for ContractEvent {
    fn to_string(&self) -> String {
        match self {
            ContractEvent::Requested { .. } => "requested".to_owned(),
            ContractEvent::Approved { .. } => "approved".to_owned(),
            ContractEvent::Rejected { .. } => "rejected".to_owned(),
            ContractEvent::Cancelled { .. } => "cancelled".to_owned(),
        }
    }
}

impl ToEvent for ContractEvent {
    fn to_event(&self) -> Result<Event> {
        let payload = serde_json::to_vec(self)?;
        Ok(Event::new("contract".to_owned(), self.to_string(), payload))
    }
}
