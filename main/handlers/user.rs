use std::sync::Arc;

use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

use identity::application::user::{GetById, Login, LoginCommand, Register, RegisterCommand};
use identity::domain::user::UserId;

use crate::handlers::context::{with_context, Context};

pub fn routes(ctx: &Arc<Context>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_by_id = warp::path!(UserId)
        .and(warp::get())
        .and(with_context(ctx.clone()))
        .and_then(get_by_id);

    let register = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_context(ctx.clone()))
        .and_then(register);

    let login = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_context(ctx.clone()))
        .and_then(login);

    warp::path("user").and(register.or(login).or(get_by_id))
}

pub async fn get_by_id(id: UserId, ctx: Arc<Context>) -> Result<impl Reply, Rejection> {
    let uc = GetById::new(ctx.user_repo());
    let res = uc.exec(&id).await.unwrap();

    Ok(warp::reply::json(&res))
}

pub async fn register(cmd: RegisterCommand, ctx: Arc<Context>) -> Result<impl Reply, Rejection> {
    let uc = Register::new(ctx.event_bus(), ctx.auth_serv(), ctx.user_repo());
    uc.exec(cmd).await.unwrap();

    Ok(StatusCode::CREATED)
}

pub async fn login(cmd: LoginCommand, ctx: Arc<Context>) -> Result<impl Reply, Rejection> {
    let uc = Login::new(ctx.auth_serv());
    let res = uc.exec(cmd).await.unwrap();

    Ok(warp::reply::json(&res))
}
