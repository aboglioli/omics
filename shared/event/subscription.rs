use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscriptionEvent {
    Created {
        id: String,
        user_id: String,
        plan_id: String,
    },
    PlanChanged {
        id: String,
        user_id: String,
        plan_id: String,
        price: f64,
    },
    PaymentRequired {
        id: String,
        user_id: String,
    },
    PaymentAdded {
        id: String,
        user_id: String,
        amount: f64,
    },
    Disabled {
        id: String,
        user_id: String,
    },
}

impl ToString for SubscriptionEvent {
    fn to_string(&self) -> String {
        match self {
            SubscriptionEvent::Created { .. } => "created".to_owned(),
            SubscriptionEvent::PlanChanged { .. } => "plan-changed".to_owned(),
            SubscriptionEvent::PaymentRequired { .. } => "payment-required".to_owned(),
            SubscriptionEvent::PaymentAdded { .. } => "payment-added".to_owned(),
            SubscriptionEvent::Disabled { .. } => "disabled".to_owned(),
        }
    }
}

impl ToEvent for SubscriptionEvent {
    fn to_event(&self) -> Result<Event> {
        Ok(Event::new(
            "subscription".to_owned(),
            self.to_string(),
            serde_json::to_value(&self)?,
        ))
    }
}
