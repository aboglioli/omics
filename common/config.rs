use std::env;

use dotenv::dotenv;

pub struct Config {
    port: u16,
    env: String,
    pagination_limit: usize,
    aws_key: String,
    aws_secret: String,
    aws_s3_bucket: String,
    aws_region: String,
}

impl Config {
    pub fn get() -> Self {
        dotenv().ok();

        Config {
            port: match env::var("PORT") {
                Ok(port) => match port.parse() {
                    Ok(port) => port,
                    _ => 3000,
                },
                _ => 3000,
            },
            env: env::var("ENV").unwrap_or_else(|_| "development".to_owned()),
            pagination_limit: 1000,
            aws_key: env::var("AWS_ACCESS_KEY_ID").unwrap_or_else(|_| "".to_owned()),
            aws_secret: env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_else(|_| "".to_owned()),
            aws_s3_bucket: env::var("AWS_S3_BUCKET").unwrap_or_else(|_| "".to_owned()),
            aws_region: env::var("AWS_REGION").unwrap_or_else(|_| "".to_owned()),
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
}
