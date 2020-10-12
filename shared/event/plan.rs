use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub enum PlanEvent {
    Created {
        id: String,
        name: String,
        description: String,
        price: f64,
    },
    NameChanged {
        id: String,
        name: String,
    },
    DescriptionChanged {
        id: String,
        description: String,
    },
    PriceChanged {
        id: String,
        price: f64,
    },
    Deleted {
        id: String,
    },
}

impl ToString for PlanEvent {
    fn to_string(&self) -> String {
        match self {
            PlanEvent::Created { .. } => "created".to_owned(),
            PlanEvent::NameChanged { .. } => "name-changed".to_owned(),
            PlanEvent::DescriptionChanged { .. } => "description-changed".to_owned(),
            PlanEvent::PriceChanged { .. } => "price-changed".to_owned(),
            PlanEvent::Deleted { .. } => "deleted".to_owned(),
        }
    }
}

impl ToEvent for PlanEvent {
    fn to_event(&self) -> Result<Event> {
        Ok(Event::new(
            "plan".to_owned(),
            self.to_string(),
            serde_json::to_value(&self)?,
        ))
    }
}
