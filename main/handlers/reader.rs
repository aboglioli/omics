use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use common::request::IncludeParams;
use payment::application::subscription::GetByReader as GetSubscriptionByReader;
use publishing::application::reader::{GetById, GetFavorites, GetFollowing};

use crate::authorization::auth;
use crate::container::MainContainer;
use crate::error::PublicError;

#[get("/{reader_id}")]
async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    let mut user_id = path.into_inner();
    if user_id == "me" {
        if let (auth_id, _) = &user_id_and_role {
            user_id = auth_id.to_string();
        }
    }

    GetById::new(c.publishing.reader_repo())
        .exec(user_id_and_role, user_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[get("/{reader_id}/following")]
async fn get_following(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    let mut user_id = path.into_inner();
    if user_id == "me" {
        if let (auth_id, _) = &user_id_and_role {
            user_id = auth_id.to_string();
        }
    }

    GetFollowing::new(c.publishing.author_repo(), c.publishing.interaction_repo())
        .exec(user_id_and_role, user_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[get("/{reader_id}/favorites")]
async fn get_favorites(
    req: HttpRequest,
    path: web::Path<String>,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    let mut user_id = path.into_inner();
    if user_id == "me" {
        if let (auth_id, _) = &user_id_and_role {
            user_id = auth_id.to_string();
        }
    }

    GetFavorites::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
        c.publishing.interaction_repo(),
        c.publishing.publication_repo(),
    )
    .exec(user_id_and_role, user_id, include.into_inner().into())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[get("/{reader_id}/subscription")]
async fn get_subscription(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    let mut user_id = path.into_inner();
    if user_id == "me" {
        if let (auth_id, _) = &user_id_and_role {
            user_id = auth_id.to_string();
        }
    }

    GetSubscriptionByReader::new(c.payment.subscription_repo())
        .exec(user_id_and_role, user_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/readers")
            .service(get_by_id)
            .service(get_following)
            .service(get_favorites)
            .service(get_subscription),
    );
}
