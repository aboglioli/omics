mod authorization;
mod container;
mod development;
mod handlers;
mod infrastructure;
mod response;

use std::error::Error;
use std::sync::Arc;

use warp::Filter;

use common::config::Config;

use container::Container;
use handlers::{author, category, collection, contract, donation, publication, subscription, user};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::get();

    // Dependencies
    let container = Arc::new(Container::new());

    if config.env() == "development" {
        println!("# Populate...");
        if let Err(err) = development::populate(&container).await {
            println!("{:?}", err);
        } else {
            println!("OK")
        }
    }

    // General
    let health = warp::path::end().map(|| "Omics");

    // Routes
    let routes = warp::path("api").and(
        health
            .or(user::routes(&container))
            .or(publication::routes(&container))
            .or(collection::routes(&container))
            .or(author::routes(&container))
            .or(category::routes(&container))
            .or(contract::routes(&container))
            .or(subscription::routes(&container))
            .or(donation::routes(&container))
            .recover(response::handle_rejection),
    );

    // Server
    println!("Listening on {}", config.port());
    warp::serve(routes).run(([0, 0, 0, 0], config.port())).await;

    Ok(())
}
