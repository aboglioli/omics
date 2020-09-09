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

    let mut user_id = path.into_inner();
    user_id = if user_id == "me" {
        auth_id.clone()
    } else {
        user_id
    };

    GetById::new(c.publishing.reader_repo(), c.publishing.user_repo())
        .exec(auth_id, user_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// GET /reader/:id/following
async fn get_following(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    let mut user_id = path.into_inner();
    user_id = if user_id == "me" {
        auth_id.clone()
    } else {
        user_id
    };

    GetFollowing::new(
        c.publishing.author_repo(),
        c.publishing.interaction_repo(),
        c.publishing.user_repo(),
    )
    .exec(auth_id, user_id)
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/readers")
            .route("/{reader_id}", web::get().to(get_by_id))
            .route("/{reader_id}/following", web::get().to(get_following)),
    );
}
