use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub enum UserEvent {
    Registered {
        id: String,
        username: String,
        email: String,
        role_id: String,
        validation_code: String,
    },
    LoggedIn {
        id: String,
        auth_token: String,
    },
    Updated {
        id: String,
        name: String,
        lastname: String,
        birthdate: Option<String>,
        gender: Option<String>,
        biography: Option<String>,
        profile_image: Option<String>,
    },
    Validated {
        id: String,
    },
    PasswordRecoveryRequested {
        id: String,
        temp_password: String,
        email: String,
    },
    RoleChanged {
        id: String,
        role_id: String,
    },
    PaymentEmailChanged {
        id: String,
        payment_email: String,
    },
    Deleted {
        id: String,
    },
}

impl ToString for UserEvent {
    fn to_string(&self) -> String {
        match self {
            UserEvent::Registered { .. } => "registered".to_owned(),
            UserEvent::LoggedIn { .. } => "logged-in".to_owned(),
            UserEvent::Updated { .. } => "updated".to_owned(),
            UserEvent::Validated { .. } => "validated".to_owned(),
            UserEvent::PasswordRecoveryRequested { .. } => "password-recovery-requested".to_owned(),
            UserEvent::RoleChanged { .. } => "role-changed".to_owned(),
            UserEvent::PaymentEmailChanged { .. } => "payment-email-changed".to_owned(),
            UserEvent::Deleted { .. } => "deleted".to_owned(),
        }
    }
}

impl ToEvent for UserEvent {
    fn to_event(&self) -> Result<Event> {
        Ok(Event::new(
            "user".to_owned(),
            self.to_string(),
            serde_json::to_value(&self)?,
        ))
    }
}
