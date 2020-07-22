use std::env;
use std::error::Error;

use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port: u16 = match env::var("PORT") {
        Ok(port) => match port.parse() {
            Ok(port) => port,
            _ => 80,
        },
        _ => 80,
    };

    let hello = warp::path::end().map(|| "Omics");

    println!("Listening on {}", port);

    warp::serve(hello).run(([0, 0, 0, 0], port)).await;

    Ok(())
}
