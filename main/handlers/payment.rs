use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::Value;

use common::error::Error;
use payment::application::payment::Validate;

use crate::container::MainContainer;
use crate::error::PublicError;

#[derive(Deserialize)]
pub struct MPEventData {
    id: String,
}

#[derive(Deserialize)]
pub struct MPEvent {
    action: String,
    data: MPEventData,
}

#[post("/mercado-pago")]
async fn mercado_pago(cmd: web::Json<Value>, c: web::Data<MainContainer>) -> impl Responder {
    let cmd = cmd.into_inner();

    let event_str = serde_json::to_string_pretty(&cmd).unwrap();
    println!("MercadoPago Event:\n{}", event_str);

    let cmd: MPEvent = serde_json::from_value(cmd)
        .map_err(Error::from)
        .map_err(PublicError::from)?;

    if cmd.action.starts_with("payment") {
        Validate::new(
            c.payment.event_pub(),
            c.payment.donation_repo(),
            c.payment.subscription_repo(),
            c.payment.payment_serv(),
        )
        .exec(cmd.data.id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
    } else {
        Ok(HttpResponse::Ok().body("not_handled"))
    }
}

#[derive(Deserialize)]
pub struct DevQuery {
    reference: String,
}

#[get("/development")]
async fn development(query: web::Query<DevQuery>, c: web::Data<MainContainer>) -> impl Responder {
    Validate::new(
        c.payment.event_pub(),
        c.payment.donation_repo(),
        c.payment.subscription_repo(),
        c.payment.payment_serv(),
    )
    .exec(query.into_inner().reference)
    .await
    .map(|_res| HttpResponse::Ok().body("Pagado"))
    .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/webhook")
            .service(mercado_pago)
            .service(development),
    );
}
