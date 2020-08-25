use std::env;

pub struct Config {
    port: u16,
    env: String,
}

impl Config {
    pub fn get() -> Self {
        Config {
            port: match env::var("PORT") {
                Ok(port) => match port.parse() {
                    Ok(port) => port,
                    _ => 80,
                },
                _ => 80,
            },
            env: env::var("ENV").unwrap_or_else(|_| "development".to_owned()),
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn env(&self) -> &str {
        &self.env
    }
}
