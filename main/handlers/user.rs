use std::sync::Arc;

use warp::http::StatusCode;
use warp::{Rejection, Reply};

use identity::application::user::{Register, RegisterCommand};

use crate::handlers::context::Context;

pub async fn register(ctx: Arc<Context>, cmd: RegisterCommand) -> Result<impl Reply, Rejection> {
    let register = Register::new(ctx.event_bus(), ctx.auth_serv(), ctx.user_repo());

    register.exec(cmd).await.unwrap();

    Ok(StatusCode::CREATED)
}
