mod application;
mod authorization;
mod container;
mod development;
mod error;
mod handlers;
mod infrastructure;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use common::config::Config;

use container::MainContainer;
use handlers::{
    author, category, collection, configuration, contract, donation, event, file, notification,
    payment, plan, publication, reader, report, role, subscription, user,
};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Omics")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = Config::get();

    // Dependencies
    let container = web::Data::new(MainContainer::new().await);
    if let Err(err) = container.subscribe().await {
        println!("Subscriptions: {}", err);
        return Ok(());
    }

    // if config.env() == "development" {
    //     if let Err(err) = development::populate(&container).await {
    //         println!("{:?}", err);
    //     }
    // }

    println!("Environment: {}", config.env());
    println!("Listening on {} ({})", config.port(), config.env());

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().finish())
            .app_data(container.clone())
            .service(
                web::scope("/api")
                    .route("/dev", web::get().to(index))
                    .configure(file::routes)
                    .configure(author::routes)
                    .configure(category::routes)
                    .configure(collection::routes)
                    .configure(event::routes)
                    .configure(publication::routes)
                    .configure(role::routes)
                    .configure(user::routes)
                    .configure(reader::routes)
                    .configure(plan::routes)
                    .configure(subscription::routes)
                    .configure(contract::routes)
                    .configure(payment::routes)
                    .configure(notification::routes)
                    .configure(report::routes)
                    .configure(donation::routes)
                    .configure(configuration::routes),
            )
    })
    .bind(format!("0.0.0.0:{}", config.port()))?
    .run()
    .await
}
