mod handlers;
mod port;


use std::error::Error;
use std::sync::Arc;

use warp::Filter;

use handlers::context::{with_context, Context};
use handlers::user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ctx = Arc::new(Context::new());

    // General
    let health = warp::path::end().map(|| "♥");

    // User
    let register = warp::path("register")
        .and(warp::post())
        .and(with_context(ctx.clone()))
        .and(warp::body::json())
        .and_then(user::register);

    let user = warp::path("user").and(register);

    let routes = warp::path("api").and(health.or(user));

    let port: u16 = port::get();
    println!("Listening on {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;

    Ok(())
}
