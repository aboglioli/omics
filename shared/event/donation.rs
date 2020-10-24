use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub enum DonationEvent {
    Created {
        id: String,
        author_id: String,
        reader_id: String,
        amount: f64,
        comment: String,
    },
    Paid {
        id: String,
        author_id: String,
        reader_id: String,
        amount: f64,
        comment: String,
    },
    Cancelled {
        id: String,
        author_id: String,
        reader_id: String,
        amount: f64,
        comment: String,
    },
}

impl ToString for DonationEvent {
    fn to_string(&self) -> String {
        match self {
            DonationEvent::Created { .. } => "created".to_owned(),
            DonationEvent::Paid { .. } => "paid".to_owned(),
            DonationEvent::Cancelled { .. } => "cancelled".to_owned(),
        }
    }
}

impl ToEvent for DonationEvent {
    fn to_event(&self) -> Result<Event> {
        Ok(Event::new(
            "donation".to_owned(),
            self.to_string(),
            serde_json::to_value(&self)?,
        ))
    }
}
