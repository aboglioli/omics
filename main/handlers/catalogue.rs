use actix_web::{web, HttpResponse, Responder};

use catalogue::application::catalogue::Get;

use crate::container::Container;
use crate::error::PublicError;

// GET /catalogue
async fn get(c: web::Data<Container>) -> impl Responder {
    Get::new(c.catalogue.catalogue_repo())
        .exec()
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/catalogue").route("", web::get().to(get)));
}
