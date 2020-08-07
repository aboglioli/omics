use std::sync::Arc;

use serde::Deserialize;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

use identity::application::user::{GetById, Login, LoginCommand, Register, RegisterCommand};
use identity::domain::user::UserId;

use crate::handlers::common::Uninmplemented;
use crate::handlers::container::{with_container, Container};

#[derive(Deserialize)]
pub struct ValidateParams {
    _code: String,
}

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_by_id = warp::get()
        .and(warp::path!(UserId))
        .and(with_container(container.clone()))
        .and_then(get_by_id);

    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(login);

    let update = warp::put()
        .and(warp::path!(UserId))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(update);

    let delete = warp::delete()
        .and(warp::path!(UserId))
        .and(with_container(container.clone()))
        .and_then(delete);

    let change_password = warp::put()
        .and(warp::path!(UserId / "password"))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(change_password);

    let recover_password = warp::post()
        .and(warp::path!(UserId / "recover-password"))
        .and(with_container(container.clone()))
        .and_then(recover_password);

    let validate = warp::post()
        .and(warp::path!(UserId))
        .and(warp::query::<ValidateParams>())
        .and(with_container(container.clone()))
        .and_then(validate);

    warp::path("users").and(
        get_by_id
            .or(register)
            .or(login)
            .or(update)
            .or(delete)
            .or(change_password)
            .or(recover_password)
            .or(validate),
    )
}

pub async fn get_by_id(id: UserId, container: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = GetById::new(container.user_repo());
    let res = uc.exec(&id).await.unwrap();

    Ok(warp::reply::json(&res))
}

pub async fn register(
    cmd: RegisterCommand,
    container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Register::new(
        container.event_bus(),
        container.auth_serv(),
        container.user_repo(),
    );
    uc.exec(cmd).await.unwrap();

    Ok(StatusCode::CREATED)
}

pub async fn login(cmd: LoginCommand, container: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = Login::new(container.auth_serv());
    let res = uc.exec(cmd).await.unwrap();

    Ok(warp::reply::json(&res))
}

pub async fn update(
    _id: UserId,
    _cmd: Uninmplemented,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn delete(_id: UserId, _container: Arc<Container>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn change_password(
    _id: UserId,
    _cmd: Uninmplemented,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn recover_password(
    _id: UserId,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn validate(
    _id: UserId,
    _params: ValidateParams,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}
