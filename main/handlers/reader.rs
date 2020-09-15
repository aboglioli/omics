use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use common::request::IncludeParams;
use publishing::application::reader::{GetById, GetFavorites, GetFollowing};

use crate::authorization::auth;
use crate::container::Container;
use crate::error::PublicError;

#[get("/{reader_id}")]
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

#[get("/{reader_id}/following")]
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

#[get("/{reader_id}/favorites")]
async fn get_favorites(
    req: HttpRequest,
    path: web::Path<String>,
    include: web::Query<IncludeParams>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    let mut user_id = path.into_inner();
    user_id = if user_id == "me" {
        auth_id.clone()
    } else {
        user_id
    };

    GetFavorites::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
        c.publishing.interaction_repo(),
        c.publishing.publication_repo(),
        c.publishing.user_repo(),
    )
    .exec(auth_id, user_id, include.into_inner().into())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/readers")
            .service(get_by_id)
            .service(get_following)
            .service(get_favorites),
    );
}
