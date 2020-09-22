use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};

use notification::application::notification::{FilterCommand, GetAll, MarkAllAsRead};

use crate::authorization::auth;
use crate::container::MainContainer;
use crate::error::PublicError;

#[get("")]
async fn get_all(
    req: HttpRequest,
    cmd: web::Query<FilterCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    GetAll::new(c.notification.notification_repo())
        .exec(auth_id, cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[post("/read")]
async fn mark_all_as_read(req: HttpRequest, c: web::Data<MainContainer>) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    MarkAllAsRead::new(c.notification.notification_repo())
        .exec(auth_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notifications")
            .service(get_all)
            .service(mark_all_as_read),
    );
}
