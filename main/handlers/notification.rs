use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

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
    let user_id_and_role = auth(&req, &c).await?;

    GetAll::new(c.notification.notification_repo())
        .exec(user_id_and_role, cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[post("/read")]
async fn mark_all_as_read(req: HttpRequest, c: web::Data<MainContainer>) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    MarkAllAsRead::new(c.notification.notification_repo())
        .exec(user_id_and_role)
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
