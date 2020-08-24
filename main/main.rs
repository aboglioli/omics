mod authorization;
mod container;
mod development;
mod error;
mod handlers;
mod infrastructure;
mod response;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use common::config::Config;

use container::Container;
use handlers::{author, catalogue, category, collection, publication, role, user};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Omics")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = Config::get();

    // Dependencies
    let container = web::Data::new(Container::new().await);
    if let Err(err) = container.subscribe().await {
        println!("Subscriptions: {}", err);
        return Ok(());
    }

    println!("Listening on {}", config.port());

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().finish())
            .app_data(container.clone())
            .service(
                web::scope("/api")
                    .route("/dev", web::get().to(index))
                    .configure(user::routes)
                    .configure(catalogue::routes)
                    .configure(publication::routes)
                    .configure(collection::routes)
                    .configure(author::routes)
                    .configure(role::routes)
                    .configure(category::routes),
            )
    })
    .bind(format!("0.0.0.0:{}", config.port()))?
    .run()
    .await
}
