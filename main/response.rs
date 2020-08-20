use serde::Serialize;
use actix_web::{http::StatusCode, error}

use common::error::{public::PublicError, Error};
use common::result::Result as CustomResult;

pub fn map<T: Serialize>(
    res: CustomResult<T>,
    ok_status: Option<StatusCode>,
) -> Result<impl Reply, Rejection> {
    match res {
        Ok(value) => Ok(warp::reply::with_status(
            warp::reply::json(&value),
            if let Some(status) = ok_status {
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

            // Safe
            let err = PublicError::from(&err, false).unwrap();

            let json = warp::reply::json(&err);
            Ok(warp::reply::with_status(json, status))
        }
    }
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(err) = err.find::<Error>() {
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

        // Safe
        let err = PublicError::from(&err, false).unwrap();

        let json = warp::reply::json(&err);
        return Ok(warp::reply::with_status(json, status));
    }

    Err(err)

    // let code;
    // let message;
    //
    // if err.is_not_found() {
    //     code = StatusCode::NOT_FOUND;
    //     message = "NOT_FOUND";
    // } else {
    //     code = StatusCode::INTERNAL_SERVER_ERROR;
    //     message = "UNHANDLED_REJECTION";
    // }
    //
    // let json = warp::reply::json(&ErrorMessage {
    //     code: code.as_u16(),
    //     message: message.into(),
    // });
    // Ok(warp::reply::with_status(json, code))
}
