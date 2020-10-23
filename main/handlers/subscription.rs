use actix_web::{delete, get, web, HttpRequest, HttpResponse, Responder};

use common::request::{IncludeParams, PaginationParams};
use payment::application::subscription::{Search, SearchCommand, Unsubscribe};

use crate::authorization::auth;
use crate::container::MainContainer;
use crate::error::PublicError;

#[get("")]
async fn search(
    req: HttpRequest,
    cmd: web::Query<SearchCommand>,
    include: web::Query<IncludeParams>,
    pagination: web::Query<PaginationParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Search::new(c.payment.subscription_repo(), c.payment.user_repo())
        .exec(
            auth_id,
            cmd.into_inner(),
            include.into_inner().into(),
            pagination.into_inner(),
        )
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
            .service(search)
            .service(unsubscribe),
    );
}
