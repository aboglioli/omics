mod business_rules;
mod service;
pub use business_rules::*;
pub use service::*;

use std::env;

use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    port: u16,
    env: String,

    pagination_limit: usize,

    aws_key: String,
    aws_secret: String,
    aws_s3_bucket: String,
    aws_region: String,

    postgres_host: String,
    postgres_port: u16,
    postgres_username: String,
    postgres_password: String,
    postgres_database: String,

    smtp_server: String,
    smtp_email: String,
    smtp_password: String,
    smtp_port: u16,

    mp_public_key: String,
    mp_access_token: String,
}

impl Config {
    pub fn get() -> Self {
        dotenv().ok();

        Config {
            port: env::var("PORT")
                .map(|port| {
                    if let Ok(port) = port.parse() {
                        port
                    } else {
                        3000
                    }
                })
                .unwrap_or_else(|_| 3000),
            env: env::var("ENV").unwrap_or_else(|_| "development".to_owned()),

            pagination_limit: 100,

            aws_key: env::var("AWS_ACCESS_KEY_ID").unwrap_or_else(|_| "".to_owned()),
            aws_secret: env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_else(|_| "".to_owned()),
            aws_s3_bucket: env::var("AWS_S3_BUCKET").unwrap_or_else(|_| "".to_owned()),
            aws_region: env::var("AWS_REGION").unwrap_or_else(|_| "".to_owned()),

            postgres_host: env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_owned()),
            postgres_port: env::var("POSTGRES_PORT")
                .map(|port| {
                    if let Ok(port) = port.parse() {
                        port
                    } else {
                        5432
                    }
                })
                .unwrap_or_else(|_| 5432),
            postgres_username: env::var("POSTGRES_USERNAME").unwrap_or_else(|_| "admin".to_owned()),
            postgres_password: env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "admin".to_owned()),
            postgres_database: env::var("POSTGRES_DATABASE").unwrap_or_else(|_| "omics".to_owned()),

            smtp_server: env::var("SMTP_SERVER").unwrap_or_else(|_| "localhost".to_owned()),
            smtp_email: env::var("SMTP_EMAIL").unwrap_or_else(|_| "user@omics.com".to_owned()),
            smtp_password: env::var("SMTP_PASSWORD").unwrap_or_else(|_| "user123".to_owned()),
            smtp_port: env::var("SMTP_PORT")
                .map(|port| {
                    if let Ok(port) = port.parse() {
                        port
                    } else {
                        25
                    }
                })
                .unwrap_or_else(|_| 25),

            mp_public_key: env::var("MP_PUBLIC_KEY").unwrap_or_else(|_| "".to_owned()),
            mp_access_token: env::var("MP_ACCESS_TOKEN").unwrap_or_else(|_| "".to_owned()),
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn env(&self) -> &str {
        &self.env
    }

    pub fn pagination_limit(&self) -> usize {
        self.pagination_limit
    }

    pub fn aws_key(&self) -> &str {
        &self.aws_key
    }

    pub fn aws_secret(&self) -> &str {
        &self.aws_secret
    }

    pub fn aws_s3_bucket(&self) -> &str {
        &self.aws_s3_bucket
    }

    pub fn aws_region(&self) -> &str {
        &self.aws_region
    }

    pub fn postgres_host(&self) -> &str {
        &self.postgres_host
    }

    pub fn postgres_port(&self) -> u16 {
        self.postgres_port
    }

    pub fn postgres_username(&self) -> &str {
        &self.postgres_username
    }

    pub fn postgres_password(&self) -> &str {
        &self.postgres_password
    }

    pub fn postgres_database(&self) -> &str {
        &self.postgres_database
    }

    pub fn smtp_server(&self) -> &str {
        &self.smtp_server
    }

    pub fn smtp_email(&self) -> &str {
        &self.smtp_email
    }

    pub fn smtp_password(&self) -> &str {
        &self.smtp_password
    }

    pub fn smtp_port(&self) -> u16 {
        self.smtp_port
    }

    pub fn mp_public_key(&self) -> &str {
        &self.mp_public_key
    }

    pub fn mp_access_token(&self) -> &str {
        &self.mp_access_token
    }
}
