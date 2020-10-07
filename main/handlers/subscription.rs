use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};

use common::request::IncludeParams;
use payment::application::subscription::{GetAll, GetByReader, Subscribe, Unsubscribe};

use crate::authorization::auth;
use crate::container::MainContainer;
use crate::error::PublicError;

#[get("")]
async fn get_all(
    req: HttpRequest,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    GetAll::new(c.payment.subscription_repo(), c.payment.user_repo())
        .exec(auth_id, include.into_inner().into())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[get("/{reader_id}/subscription")]
async fn get_by_reader(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    let mut user_id = path.into_inner();
    user_id = if user_id == "me" {
        auth_id.clone()
    } else {
        user_id
    };

    GetByReader::new(c.payment.subscription_repo())
        .exec(auth_id, user_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[post("/{plan_id}/subscribe")]
async fn subscribe(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Subscribe::new(
        c.payment.event_pub(),
        c.payment.plan_repo(),
        c.payment.reader_repo(),
        c.payment.subscription_repo(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[delete("")]
async fn unsubscribe(req: HttpRequest, c: web::Data<MainContainer>) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Unsubscribe::new(c.payment.event_pub(), c.payment.subscription_repo())
        .exec(auth_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/subscriptions")
            .service(get_all)
            .service(unsubscribe),
    )
    .service(web::scope("/plans").service(subscribe))
    .service(web::scope("/readers").service(get_by_reader));
}
