use actix_web::{delete, get, web, HttpRequest, HttpResponse, Responder};

use common::request::IncludeParams;
use payment::application::subscription::{GetAll, Unsubscribe};

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
    );
}
