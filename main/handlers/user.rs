use actix_web::{web, HttpRequest, HttpResponse, Responder};

use common::request::IncludeParams;
use identity::application::user::{
    ChangePassword, ChangePasswordCommand, ChangeRole, ChangeRoleCommand, Delete, GetById, Login,
    LoginCommand, RecoverPassword, RecoverPasswordCommand, Register, RegisterCommand, Search,
    SearchCommand, Update, UpdateCommand, Validate,
};

use crate::authorization::auth;
use crate::container::Container;
use crate::error::PublicError;

// POST /register
async fn register(cmd: web::Json<RegisterCommand>, c: web::Data<Container>) -> impl Responder {
    Register::new(
        c.identity.event_pub(),
        c.identity.user_repo(),
        c.identity.user_serv(),
    )
    .exec(cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// POST /login
async fn login(cmd: web::Json<LoginCommand>, c: web::Data<Container>) -> impl Responder {
    Login::new(c.identity.event_pub(), c.identity.authentication_serv())
        .exec(cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// POST /recover-password
async fn recover_password(
    cmd: web::Json<RecoverPasswordCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    RecoverPassword::new(
        c.identity.event_pub(),
        c.identity.user_repo(),
        c.identity.user_serv(),
    )
    .exec(cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// GET /users?role_id=...
async fn search(
    req: HttpRequest,
    cmd: web::Query<SearchCommand>,
    include: web::Query<IncludeParams>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Search::new(c.identity.user_repo())
        .exec(auth_id, cmd.into_inner(), include.into_inner().into())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// GET /users/:id
async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    include: web::Query<IncludeParams>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    GetById::new(c.identity.user_repo())
        .exec(auth_id, path.into_inner(), include.into_inner().into())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// PUT /users/:id
async fn update(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<UpdateCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Update::new(c.identity.event_pub(), c.identity.user_repo())
        .exec(auth_id, path.into_inner(), cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// DELETE /users/:id
async fn delete(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Delete::new(c.identity.event_pub(), c.identity.user_repo())
        .exec(auth_id, path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// PUT /users/:id/password
async fn change_password(
    _req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<ChangePasswordCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    ChangePassword::new(c.identity.user_serv())
        .exec(path.into_inner(), cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// GET /users/:id/validate/:code
async fn validate(path: web::Path<(String, String)>, c: web::Data<Container>) -> impl Responder {
    let path = path.into_inner();
    Validate::new(c.identity.event_pub(), c.identity.user_repo())
        .exec(path.0, path.1)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// PUT /users/:id/role
async fn change_role(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<ChangeRoleCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    ChangeRole::new(c.identity.role_repo(), c.identity.user_repo())
        .exec(auth_id, path.into_inner(), cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        .route("/recover-password", web::post().to(recover_password))
        .service(
            web::scope("/users")
                .route("", web::get().to(search))
                .route("/{user_id}", web::get().to(get_by_id))
                .route("/{user_id}", web::put().to(update))
                .route("/{user_id}", web::delete().to(delete))
                .route("/{user_id}/password", web::put().to(change_password))
                .route(
                    "/{user_id}/validate/{validation_code}",
                    web::get().to(validate),
                )
                .route("/{user_id}/role", web::put().to(change_role)),
        );
}
