use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use common::request::{IncludeParams, PaginationParams};
use payment::application::donation::{Charge, GetById, Search, SearchCommand};

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
    let user_id_and_role = auth(&req, &c).await?;

    Search::new(
        c.publishing.author_repo(),
        c.payment.donation_repo(),
        c.publishing.reader_repo(),
    )
    .exec(
        user_id_and_role,
        cmd.into_inner(),
        include.into_inner().into(),
        pagination.into_inner(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[get("/{donation_id}")]
async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    GetById::new(
        c.publishing.author_repo(),
        c.payment.donation_repo(),
        c.publishing.reader_repo(),
    )
    .exec(
        user_id_and_role,
        path.into_inner(),
        include.into_inner().into(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[post("/charge")]
async fn charge(req: HttpRequest, c: web::Data<MainContainer>) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    Charge::new(
        c.payment.event_pub(),
        c.payment.donation_repo(),
        c.identity.user_repo(),
        c.payment.payment_serv(),
    )
    .exec(user_id_and_role)
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/donations")
            .service(search)
            .service(get_by_id)
            .service(charge),
    );
}
