use serde::{Deserialize, Serialize};

use common::error::Error;
use common::event::{Event, ToEvent};
use common::result::Result;

use crate::domain::user::UserId;

#[derive(Debug, Serialize, Deserialize)]
pub enum UserEvent {
    Registered {
        id: UserId,
        username: String,
        email: String,
    },
    LoggedIn {
        id: UserId,
    },
    Updated {
        id: UserId,
        name: String,
        lastname: String,
    },
    Validated {
        id: UserId,
    },
    PasswordRecoveryRequested {
        id: UserId,
        temp_password: String,
        email: String,
    },
}

impl ToEvent for UserEvent {
    fn to_event(&self) -> Result<Event> {
        let payload = match serde_json::to_vec(&self) {
            Ok(vec) => vec,
            Err(err) => return Err(Error::new("user", "event").wrap_raw(err).build()),
        };

        let event = match self {
            UserEvent::Registered { .. } => Event::new("user", "registered", payload),
            UserEvent::LoggedIn { .. } => Event::new("user", "logged-in", payload),
            UserEvent::Updated { .. } => Event::new("user", "updated", payload),
            UserEvent::Validated { .. } => Event::new("user", "validated", payload),
            UserEvent::PasswordRecoveryRequested { .. } => {
                Event::new("user", "password-recovery-requested", payload)
            }
        };

        Ok(event)
    }
}
