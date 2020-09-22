use std::sync::Arc;

use async_trait::async_trait;

use common::event::{Event, EventHandler};
use common::result::Result;
use shared::event::UserEvent;

use crate::domain::email::{Email, EmailService};

pub struct RegisteredHandler {
    email_serv: Arc<dyn EmailService>,
}

impl RegisteredHandler {
    pub fn new(email_serv: Arc<dyn EmailService>) -> Self {
        RegisteredHandler { email_serv }
    }
}

#[async_trait]
impl EventHandler for RegisteredHandler {
    fn topic(&self) -> &str {
        "user"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event: UserEvent = serde_json::from_slice(event.payload())?;

        match event {
            UserEvent::Registered {
                id,
                username,
                email,
                validation_code,
            } => {
                let email = Email::new(
                    email,
                    format!("Bienvenido {}", username),
                    format!(
                        r#"
                        <p>
                            <b>Hola</b>.
                        </p>

                        <p>
                            Valida tu cuenta haciendo click en el siguiente <a href="http://localhost:3000/api/users/{}/validate/{}">enlace</a>.
                        </p>

                        <p>
                            Un place tenerte con nosotros.<br>
                            <i>El equipo de Omics</i>.
                        </p>
                        "#,
                        id, validation_code,
                    ),
                )?;

                self.email_serv.send(&email).await?;
            }
            UserEvent::PasswordRecoveryRequested { id, temp_password, email } => {
                let email = Email::new(
                    email,
                    "Recuperar contraseña".to_owned(),
                    format!(
                        r#"
                        <p>
                            <b>Hola</b>.
                        </p>
                        <p>
                            Recupera tu contraseña desde el siguiente
                            <a href="http://localhost:4200/home/{}/recover-password/{}">enlace</a>.
                        </p>
                        "#,
                        id, temp_password,
                    ),
                )?;

                self.email_serv.send(&email).await?;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
