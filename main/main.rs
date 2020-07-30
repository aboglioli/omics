mod handlers;
mod port;

use std::error::Error;
use std::sync::Arc;

use warp::Filter;

use handlers::context::Context;
use handlers::user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ctx = Arc::new(Context::new());

    // General
    let health = warp::path::end().map(|| "Omics");

    // Routes
    let routes = warp::path("api").and(health.or(user::routes(&ctx)));

    // Server
    let port: u16 = port::get();
    println!("Listening on {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;

    Ok(())
}
