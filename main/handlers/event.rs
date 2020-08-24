use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use common::event::EventRepository;

use crate::authorization::auth;
use crate::container::Container;
use crate::error::PublicError;

#[derive(Serialize)]
pub struct PublicEvent {
    pub id: String,
    pub timestamp: String,
    pub payload: String,
}

#[derive(Serialize)]
pub struct GetAllResponse {
    pub events: Vec<PublicEvent>,
}

// GET /events
async fn get(req: HttpRequest, c: web::Data<Container>) -> impl Responder {
    let _auth_id = auth(&req, &c).await?;

    c.event_repo()
        .find_all()
        .await
        .map(|events| {
            events
                .into_iter()
                .map(|event| PublicEvent {
                    id: event.id().to_string(),
                    timestamp: event.timestamp().to_string(),
                    payload: String::from_utf8_lossy(event.payload()).into_owned(),
                })
                .collect()
        })
        .map(|events| HttpResponse::Ok().json(GetAllResponse { events }))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/events").route("", web::get().to(get)));
}
