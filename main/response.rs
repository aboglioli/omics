use serde::Serialize;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use common::error::public::PublicError;
use common::result::Result as CustomResult;

pub fn check<T: Serialize>(
    res: CustomResult<T>,
    status: Option<StatusCode>,
) -> Result<impl Reply, Rejection> {
    match res {
        Ok(value) => Ok(warp::reply::with_status(
            warp::reply::json(&value),
            if let Some(status) = status {
                status
            } else {
                StatusCode::OK
            },
        )),
        Err(err) => {
            let status = match err.status() {
                Some(status) => {
                    if let Ok(status) = StatusCode::from_u16(status as u16) {
                        status
                    } else {
                        StatusCode::BAD_REQUEST
                    }
                }
                None => StatusCode::BAD_REQUEST,
            };
            let err = PublicError::from(&err, false).unwrap(); // TODO: safe
            let json = warp::reply::json(&err);
            Ok(warp::reply::with_status(json, status))
        }
    }
}
