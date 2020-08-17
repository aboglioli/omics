use std::env;

pub struct Config {
    pub port: u16,
    pub env: String,
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
            env: env::var("ENV").unwrap_or("development".to_owned()),
        }
    }
}
