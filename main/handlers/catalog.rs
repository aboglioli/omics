use actix_web::{get, web, HttpResponse, Responder};

use publishing::application::catalog::GetCatalog;

use crate::container::MainContainer;
use crate::error::PublicError;

#[get("")]
async fn get(c: web::Data<MainContainer>) -> impl Responder {
    GetCatalog::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
        c.publishing.publication_repo(),
    )
    .exec()
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/catalog").service(get));
}
