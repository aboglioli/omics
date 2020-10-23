use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use reports::application::report::{Generate, GenerateCommand};

use crate::authorization::auth;
use crate::container::MainContainer;
use crate::error::PublicError;

#[get("")]
async fn generate(
    req: HttpRequest,
    cmd: web::Query<GenerateCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Generate::new(
        c.publishing.author_repo(),
        c.payment.contract_repo(),
        c.publishing.publication_repo(),
        c.payment.subscription_repo(),
        c.identity.user_repo(),
    )
    .exec(auth_id, cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/reports").service(generate));
}
