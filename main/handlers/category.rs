use actix_web::{web, HttpResponse, Responder};

use publishing::application::category::GetAll;

use crate::container::Container;
use crate::error::PublicError;

async fn get_all(c: web::Data<Container>) -> impl Responder {
    GetAll::new(c.publishing.category_repo())
        .exec()
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/categories").route("", web::get().to(get_all)));
}
