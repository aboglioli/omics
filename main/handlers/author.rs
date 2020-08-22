use actix_web::{web, HttpRequest, HttpResponse, Responder};

use publishing::application::author::{GetAll, GetById};

use crate::authorization::auth;
use crate::container::Container;
use crate::error::PublicError;

async fn get_all(req: HttpRequest, c: web::Data<Container>) -> impl Responder {
    let _user_id = auth(&req, &c).await?;

    GetAll::new(
        c.publishing.author_repo(),
        c.publishing.collection_repo(),
        c.publishing.publication_repo(),
    )
    .exec()
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let _user_id = auth(&req, &c).await?;

    GetById::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
    )
    .exec(path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/authors")
            .route("", web::get().to(get_all))
            .route("/{author_id}", web::get().to(get_by_id)),
    );
}
