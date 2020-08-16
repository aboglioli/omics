use std::sync::Arc;

use serde::Deserialize;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

use identity::application::user::{
    GetById, Login, LoginCommand, Register, RegisterCommand, Update, UpdateCommand, Validate,
};

use crate::authorization;
use crate::container::{with_container, Container};
use crate::handlers::common::Uninmplemented;
use crate::response;

#[derive(Deserialize)]
pub struct ValidateParams {
    _code: String,
}

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_by_id = warp::get()
        .and(warp::path!(String))
        .and(warp::header::<String>("authorization"))
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
        .and(warp::path!(String))
        .and(warp::body::json())
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(update);

    let delete = warp::delete()
        .and(warp::path!(String))
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(delete);

    let change_password = warp::put()
        .and(warp::path!(String / "change-password"))
        .and(warp::body::json())
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(change_password);

    let recover_password = warp::post()
        .and(warp::path!(String / "recover-password"))
        .and(with_container(container.clone()))
        .and_then(recover_password);

    let validate = warp::get()
        .and(warp::path!(String / "validate" / String))
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

pub async fn get_by_id(
    id: String,
    authorization_header: String,
    container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let token = authorization::extract_token(&authorization_header).unwrap();
    let authorization_serv = container.identity.authorization_serv();
    let user = authorization_serv.authorize(&token).await.unwrap();

    let uc = GetById::new(container.identity.user_repo());
    let res = uc.exec(user.base().id().value().to_owned(), id).await;

    response::check(res, None)
}

pub async fn register(
    cmd: RegisterCommand,
    container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Register::new(
        container.identity.event_pub(),
        container.identity.user_repo(),
        container.identity.user_serv(),
    );
    let res = uc.exec(cmd).await;

    response::check(res, Some(StatusCode::CREATED))
}

pub async fn login(cmd: LoginCommand, container: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = Login::new(
        container.identity.event_pub(),
        container.identity.authentication_serv(),
    );
    let res = uc.exec(cmd).await;

    response::check(res, None)
}

pub async fn update(
    id: String,
    cmd: UpdateCommand,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let token = authorization::extract_token(&authorization_header).unwrap();
    let authorization_serv = c.identity.authorization_serv();
    let _user = authorization_serv.authorize(&token).await.unwrap();
    let uc = Update::new(c.identity.event_pub(), c.identity.user_repo());

    let res = uc.exec(id, cmd).await;

    response::check(res, None)
}

pub async fn delete(
    _id: String,
    authorization_header: String,
    container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let token = authorization::extract_token(&authorization_header).unwrap();
    let authorization_serv = container.identity.authorization_serv();
    let _user = authorization_serv.authorize(&token).await.unwrap();

    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn change_password(
    _id: String,
    _cmd: Uninmplemented,
    authorization_header: String,
    container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let token = authorization::extract_token(&authorization_header).unwrap();
    let authorization_serv = container.identity.authorization_serv();
    let _user = authorization_serv.authorize(&token).await.unwrap();

    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn recover_password(
    _id: String,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn validate(
    id: String,
    code: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Validate::new(c.identity.event_pub(), c.identity.user_repo());
    let res = uc.exec(id, code).await;

    response::check(res, None)
}
