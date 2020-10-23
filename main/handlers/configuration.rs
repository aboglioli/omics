use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};

use crate::application::configuration::Get;
use crate::authorization::auth;
use crate::container::MainContainer;
use crate::error::PublicError;

#[get("")]
async fn get_all(req: HttpRequest, c: web::Data<MainContainer>) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Get::new(c.identity.user_repo(), c.config_serv())
        .exec(auth_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/configuration").service(get_all));
}
