use async_trait::async_trait;
use lettre::message::header::ContentType;
use lettre::message::{Message, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};

use common::config::Config;
use common::result::Result;

use crate::domain::email::{Email, EmailService};

pub struct GmailService;

impl GmailService {
    pub fn new() -> Self {
        GmailService
    }
}

#[async_trait]
impl EmailService for GmailService {
    async fn send(&self, email: &Email) -> Result<()> {
        let config = Config::get();

        let email = Message::builder()
            .from(
                format!("Equipo de Omics <{}>", config.smtp_email())
                    .parse()
                    .unwrap(),
            )
            .to(format!("{} <{}>", email.to(), email.to()).parse().unwrap())
            .subject(email.title())
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::html())
                    .body(email.body()),
            )
            .unwrap();

        let creds = Credentials::new(
            config.smtp_email().to_string(),
            config.smtp_password().to_string(),
        );

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(config.smtp_server())
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }

        Ok(())
    }
}
