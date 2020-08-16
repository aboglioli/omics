mod authorization;
mod container;
mod handlers;
mod port;
mod response;

use std::error::Error;
use std::sync::Arc;

use warp::Filter;

use container::Container;
use handlers::{contract, donation, publication, subscription, user};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let container = Arc::new(Container::new());

    // General
    let health = warp::path::end().map(|| "Omics");

    // Routes
    let routes = warp::path("api").and(
        health
            .or(user::routes(&container))
            .or(publication::routes(&container))
            .or(contract::routes(&container))
            .or(subscription::routes(&container))
            .or(donation::routes(&container)),
    );

    // Server
    let port: u16 = port::get();
    println!("Listening on {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;

    Ok(())
}
