use actix_web::{HttpResponse, Responder};
use serde::Serialize;

use common::result::Result;

use crate::error::PublicError;

pub fn map<T: Serialize>(res: Result<T>) -> impl Responder {
    res.map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}
