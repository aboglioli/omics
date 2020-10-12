use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};

use common::request::{IncludeParams, PaginationParams};
use payment::application::contract::{
    Approve, Cancel, ChargeForContract, GenerateStatistics, GenerateStatisticsCommand, Reject,
    Search, SearchCommand,
};

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
        c.payment.contract_repo(),
        c.payment.publication_repo(),
        c.payment.user_repo(),
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

#[post("/{contract_id}/approve")]
async fn approve(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Approve::new(
        c.payment.event_pub(),
        c.payment.contract_repo(),
        c.payment.user_repo(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[post("/{contract_id}/reject")]
async fn reject(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Reject::new(
        c.payment.event_pub(),
        c.payment.contract_repo(),
        c.payment.user_repo(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[delete("/{contract_id}")]
async fn cancel(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Cancel::new(
        c.payment.event_pub(),
        c.payment.contract_repo(),
        c.payment.publication_repo(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[post("/statistics")]
async fn generate_statistics(
    req: HttpRequest,
    cmd: web::Json<GenerateStatisticsCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    GenerateStatistics::new(
        c.payment.event_pub(),
        c.payment.contract_repo(),
        c.payment.user_repo(),
        c.payment.contract_serv(),
    )
    .exec(auth_id, cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[post("/{contract_id}/charge")]
async fn charge(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    ChargeForContract::new(
        c.payment.event_pub(),
        c.payment.contract_repo(),
        c.payment.publication_repo(),
        c.payment.payment_serv(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/contracts")
            .service(search)
            .service(approve)
            .service(reject)
            .service(cancel)
            .service(generate_statistics)
            .service(charge),
    );
}
