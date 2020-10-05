use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub enum PlanEvent {
    Created { id: String, price: f64 },
    PriceChanged { id: String, price: f64 },
    Deleted { id: String },
}

impl ToString for PlanEvent {
    fn to_string(&self) -> String {
        match self {
            PlanEvent::Created { .. } => "created".to_owned(),
            PlanEvent::PriceChanged { .. } => "price-changed".to_owned(),
            PlanEvent::Deleted { .. } => "deleted".to_owned(),
        }
    }
}

impl ToEvent for PlanEvent {
    fn to_event(&self) -> Result<Event> {
        let payload = serde_json::to_vec(self)?;
        Ok(Event::new("plan".to_owned(), self.to_string(), payload))
    }
}
