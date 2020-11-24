use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};

use payment::application::plan::{Create, CreateCommand, Delete, GetAll, Update, UpdateCommand};
use payment::application::subscription::Subscribe;

use crate::authorization::auth;
use crate::container::MainContainer;
use crate::error::PublicError;

#[get("")]
async fn get_all(c: web::Data<MainContainer>) -> impl Responder {
    GetAll::new(c.payment.plan_repo())
        .exec()
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[post("")]
async fn create(
    req: HttpRequest,
    cmd: web::Json<CreateCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    Create::new(c.payment.event_pub(), c.payment.plan_repo())
        .exec(user_id_and_role, cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[put("/{plan_id}")]
async fn update(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<UpdateCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    Update::new(c.payment.event_pub(), c.payment.plan_repo())
        .exec(user_id_and_role, path.into_inner(), cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[delete("/{plan_id}")]
async fn delete(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    Delete::new(
        c.payment.event_pub(),
        c.payment.plan_repo(),
        c.payment.subscription_repo(),
    )
    .exec(user_id_and_role, path.into_inner())
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
    let user_id_and_role = auth(&req, &c).await?;

    Subscribe::new(
        c.payment.event_pub(),
        c.payment.plan_repo(),
        c.payment.reader_repo(),
        c.payment.subscription_repo(),
        c.payment.user_repo(),
        c.payment.payment_serv(),
    )
    .exec(user_id_and_role, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/plans")
            .service(get_all)
            .service(create)
            .service(update)
            .service(delete)
            .service(subscribe),
    );
}
