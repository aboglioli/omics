use std::env;

pub fn get() -> u16 {
    match env::var("PORT") {
        Ok(port) => match port.parse() {
            Ok(port) => port,
            _ => 80,
        },
        _ => 80,
    }
}
