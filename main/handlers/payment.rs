use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use serde_json::Value;

use crate::container::MainContainer;

#[post("/mercado-pago")]
async fn mercado_pago(
    _req: HttpRequest,
    cmd: web::Json<Value>,
    _c: web::Data<MainContainer>,
) -> impl Responder {
    let cmd = serde_json::to_string_pretty(&cmd.into_inner()).unwrap();

    println!("{}", cmd);

    HttpResponse::Ok()
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/webhook").service(mercado_pago));
}
