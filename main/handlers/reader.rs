use actix_web::{web, HttpRequest, HttpResponse, Responder};

use publishing::application::reader::{GetById, GetFollowing};

use crate::authorization::auth;
use crate::container::Container;
use crate::error::PublicError;

// GET /reader/:id
async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    GetById::new(c.publishing.reader_repo())
        .exec(auth_id, path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

async fn get_following(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    GetFollowing::new(c.publishing.author_repo(), c.publishing.interaction_repo())
        .exec(auth_id, path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reader")
            .route("/{reader_id}", web::get().to(get_by_id))
            .route("/{reader_id}/following", web::get().to(get_following)),
    );
}
