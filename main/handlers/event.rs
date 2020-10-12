use actix_web::{get, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use common::event::EventRepository;

use crate::container::MainContainer;
use crate::error::PublicError;

#[derive(Serialize)]
pub struct PublicEvent {
    pub id: String,
    pub topic: String,
    pub code: String,
    pub timestamp: String,
    pub payload: Value,
}

#[derive(Deserialize)]
pub struct SearchCommand {
    pub topic: Option<String>,
    pub code: Option<String>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct GetAllResponse {
    pub events: Vec<PublicEvent>,
}

// GET /events
#[get("")]
async fn get(cmd: web::Query<SearchCommand>, c: web::Data<MainContainer>) -> impl Responder {
    c.event_repo()
        .search(
            None,
            cmd.topic.as_ref(),
            cmd.code.as_ref(),
            cmd.from.as_ref(),
            cmd.to.as_ref(),
        )
        .await
        .map(|events| {
            events
                .into_iter()
                .map(|event| PublicEvent {
                    id: event.id().to_string(),
                    topic: event.topic().to_string(),
                    code: event.code().to_string(),
                    timestamp: event.timestamp().to_rfc3339(),
                    payload: serde_json::from_value(event.payload()).unwrap(),
                })
                .collect()
        })
        .map(|events| HttpResponse::Ok().json(GetAllResponse { events }))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/events").service(get));
}
