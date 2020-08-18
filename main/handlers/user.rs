use std::sync::Arc;

use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

use identity::application::user::{
    ChangePassword, ChangePasswordCommand, ChangeRole, ChangeRoleCommand, Delete, GetAll, GetById,
    Login, LoginCommand, RecoverPassword, Register, RegisterCommand, Update, UpdateCommand,
    Validate,
};

use crate::authorization::with_auth;
use crate::container::{with_container, Container};
use crate::response;

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /users
    let _get_all = warp::get()
        .and(warp::path::end())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(get_all);

    // GET /users/:id
    let get_by_id = warp::get()
        .and(warp::path!(String))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(get_by_id);

    // POST /users/register
    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(register);

    // POST /users/login
    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(login);

    // PUT /users/:id
    let update = warp::put()
        .and(warp::path!(String))
        .and(warp::body::json())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(update);

    // DELETE /users/:id
    let delete = warp::delete()
        .and(warp::path!(String))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(delete);

    // PUT /users/:id/change-password
    let change_password = warp::put()
        .and(warp::path!(String / "change-password"))
        .and(warp::body::json())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(change_password);

    // POST /users/:id/recover-password
    let recover_password = warp::get()
        .and(warp::path!(String / "recover-password"))
        .and(with_container(container.clone()))
        .and_then(recover_password);

    // GET /users/:id/validate/:code
    let validate = warp::get()
        .and(warp::path!(String / "validate" / String))
        .and(with_container(container.clone()))
        .and_then(validate);

    // POST /users/:id/role
    let change_role = warp::put()
        .and(warp::path!(String / "role"))
        .and(warp::body::json())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(change_role);

    warp::path("users").and(
        get_by_id
            .or(register)
            .or(login)
            .or(update)
            .or(delete)
            .or(change_password)
            .or(recover_password)
            .or(validate)
            .or(change_role),
    )
}

pub async fn get_all(user_id: String, c: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = GetAll::new(c.identity.user_repo());
    let res = uc.exec(user_id).await;

    response::map(res, None)
}

pub async fn get_by_id(
    id: String,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = GetById::new(c.identity.user_repo());
    let res = uc.exec(user_id, id).await;

    response::map(res, None)
}

pub async fn register(cmd: RegisterCommand, c: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = Register::new(
        c.identity.event_pub(),
        c.identity.user_repo(),
        c.identity.user_serv(),
    );
    let res = uc.exec(cmd).await;

    response::map(res, Some(StatusCode::CREATED))
}

pub async fn login(cmd: LoginCommand, c: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = Login::new(c.identity.event_pub(), c.identity.authentication_serv());
    let res = uc.exec(cmd).await;

    response::map(res, None)
}

pub async fn update(
    id: String,
    cmd: UpdateCommand,
    _user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Update::new(c.identity.event_pub(), c.identity.user_repo());
    let res = uc.exec(id, cmd).await;

    response::map(res, None)
}

pub async fn delete(
    id: String,
    _user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Delete::new(c.identity.event_pub(), c.identity.user_repo());
    let res = uc.exec(id).await;

    response::map(res, None)
}

pub async fn change_password(
    id: String,
    cmd: ChangePasswordCommand,
    _user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = ChangePassword::new(c.identity.user_serv());
    let res = uc.exec(id, cmd).await;

    response::map(res, None)
}

pub async fn recover_password(email: String, c: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = RecoverPassword::new(
        c.identity.event_pub(),
        c.identity.user_repo(),
        c.identity.user_serv(),
    );
    let res = uc.exec(email).await;

    response::map(res, None)
}

pub async fn validate(
    id: String,
    code: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Validate::new(c.identity.event_pub(), c.identity.user_repo());
    let res = uc.exec(id, code).await;

    response::map(res, None)
}

pub async fn change_role(
    id: String,
    cmd: ChangeRoleCommand,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = ChangeRole::new(c.identity.role_repo(), c.identity.user_repo());
    let res = uc.exec(user_id, id, cmd).await;

    response::map(res, None)
}
