mod authorization;
mod container;
mod development;
mod events;
mod handlers;
mod infrastructure;
mod response;

use std::error::Error;
use std::sync::Arc;

use warp::http::header::{HeaderMap, HeaderValue};
use warp::Filter;

use common::config::Config;

use container::Container;
use handlers::{
    author, catalogue, category, collection, contract, donation, publication, role, subscription,
    user,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let config = Config::get();

    // Dependencies
    let container = Arc::new(Container::new());

    if config.env() == "development" {
        if let Err(err) = development::run(&container).await {
            println!("{:?}", err);
        }
    }

    if let Err(err) = events::subscribe(&container).await {
        println!("Subscriptions: {:?}", err);
    }

    // CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Content-Type",
            "Access-Control-Allow-Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
        ])
        .allow_methods(vec!["GET", "POST", "DELETE", "PUT", "OPTIONS"])
        .allow_credentials(true);

    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET, POST, OPTIONS, PUT, PATCH, DELETE"),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("X-Requested-With,content-type"),
    );
    headers.insert(
        "Access-Control-Allow-Credentials",
        HeaderValue::from_static("true"),
    );

    // General
    let health = warp::path::end().map(|| "Omics");

    // Routes
    let routes = warp::path("api")
        .and(
            health
                .or(development::routes(&container))
                .or(catalogue::routes(&container))
                .or(role::routes(&container))
                .or(user::routes(&container))
                .or(publication::routes(&container))
                .or(collection::routes(&container))
                .or(author::routes(&container))
                .or(category::routes(&container))
                .or(contract::routes(&container))
                .or(subscription::routes(&container))
                .or(donation::routes(&container))
                .recover(response::handle_rejection),
        )
        .with(cors)
        .with(warp::reply::with::headers(headers))
        .with(warp::log("cors test"));

    // let routes = warp::any().and(routes).with().with(cors).with(warp::log("cors test"));

    // Server
    println!("Listening on {}", config.port());
    warp::serve(routes).run(([0, 0, 0, 0], config.port())).await;

    Ok(())
}
