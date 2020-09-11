use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use serde_json::Value;

use common::event::EventRepository;

use crate::container::Container;
use crate::error::PublicError;

#[derive(Serialize)]
pub struct PublicEvent {
    pub id: String,
    pub topic: String,
    pub code: String,
    pub timestamp: String,
    pub payload: Value,
}

#[derive(Serialize)]
pub struct GetAllResponse {
    pub events: Vec<PublicEvent>,
}

// GET /events
async fn get(c: web::Data<Container>) -> impl Responder {
    c.event_repo()
        .find_all()
        .await
        .map(|events| {
            events
                .into_iter()
                .map(|event| PublicEvent {
                    id: event.id().to_string(),
                    topic: event.topic().to_string(),
                    code: event.code().to_string(),
                    timestamp: event.timestamp().to_string(),
                    payload: serde_json::from_slice(event.payload()).unwrap(),
                })
                .collect()
        })
        .map(|events| HttpResponse::Ok().json(GetAllResponse { events }))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/events").route("", web::get().to(get)));
}
