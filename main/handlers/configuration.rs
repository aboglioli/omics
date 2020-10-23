use actix_web::{get, put, web, HttpRequest, HttpResponse, Responder};

use crate::application::configuration::{Get, Update};
use crate::application::dtos::ConfigurationDto;
use crate::authorization::auth;
use crate::container::MainContainer;
use crate::error::PublicError;

#[get("")]
async fn get_all(c: web::Data<MainContainer>) -> impl Responder {
    Get::new(c.config_serv())
        .exec()
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[put("")]
async fn update(
    req: HttpRequest,
    cmd: web::Json<ConfigurationDto>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Update::new(c.identity.user_repo(), c.config_serv())
        .exec(auth_id, cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/configuration")
            .service(get_all)
            .service(update),
    );
}
