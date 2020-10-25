use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use common::request::{IncludeParams, PaginationParams};
use payment::application::donation::{Charge, Search, SearchCommand};

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

    Search::new(
        c.publishing.author_repo(),
        c.payment.donation_repo(),
        c.publishing.reader_repo(),
        c.identity.user_repo(),
    )
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

#[post("/charge")]
async fn charge(
    req: HttpRequest,
    _path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Charge::new(c.payment.event_pub(), c.payment.donation_repo())
        .exec(auth_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/donations").service(search).service(charge));
}
