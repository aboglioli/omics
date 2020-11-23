use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use crate::application::backup::{Generate, List};

use crate::authorization::auth;
use crate::container::MainContainer;
use crate::error::PublicError;

#[get("")]
async fn list_files(req: HttpRequest, c: web::Data<MainContainer>) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    List::new()
        .exec(user_id_and_role)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[post("")]
async fn generate_backup(req: HttpRequest, c: web::Data<MainContainer>) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    Generate::new()
        .exec(user_id_and_role)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/backup")
            .service(list_files)
            .service(generate_backup),
    );
}
