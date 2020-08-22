use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};

use identity::application::user::{
    ChangePassword, ChangePasswordCommand, ChangeRole, ChangeRoleCommand, Delete, GetAll, GetById,
    Login, LoginCommand, RecoverPassword, RecoverPasswordCommand, Register, RegisterCommand,
    Update, UpdateCommand, Validate,
};

use crate::authorization::auth;
use crate::container::Container;
use crate::error::PublicError;

async fn register(
    cmd: web::Json<RegisterCommand>,
    c: web::Data<Container>,
) -> Result<HttpResponse, PublicError> {
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

async fn login(
    cmd: web::Json<LoginCommand>,
    c: web::Data<Container>,
) -> Result<HttpResponse, PublicError> {
    Login::new(c.identity.event_pub(), c.identity.authentication_serv())
        .exec(cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

async fn recover_password(
    cmd: web::Json<RecoverPasswordCommand>,
    c: web::Data<Container>,
) -> Result<HttpResponse, PublicError> {
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

async fn get_all(req: HttpRequest, c: web::Data<Container>) -> Result<HttpResponse, PublicError> {
    let user_id = auth(&req, &c).await?;

    GetAll::new(c.identity.user_repo())
        .exec(user_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let user_id = auth(&req, &c).await?;

    let id = path.into_inner();
    GetById::new(c.identity.user_repo())
        .exec(user_id, id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

async fn update(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<UpdateCommand>,
    c: web::Data<Container>,
) -> Result<HttpResponse, PublicError> {
    let _user_id = auth(&req, &c).await?;

    Update::new(c.identity.event_pub(), c.identity.user_repo())
        .exec(path.into_inner(), cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

async fn delete(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> Result<HttpResponse, PublicError> {
    let _user_id = auth(&req, &c).await?;

    Delete::new(c.identity.event_pub(), c.identity.user_repo())
        .exec(path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

async fn change_password(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<ChangePasswordCommand>,
    c: web::Data<Container>,
) -> Result<HttpResponse, PublicError> {
    let _user_id = auth(&req, &c).await?;

    ChangePassword::new(c.identity.user_serv())
        .exec(path.into_inner(), cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

async fn validate(
    path: web::Path<(String, String)>,
    c: web::Data<Container>,
) -> Result<HttpResponse, PublicError> {
    let path = path.into_inner();
    Validate::new(c.identity.event_pub(), c.identity.user_repo())
        .exec(path.0, path.1)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

async fn change_role(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<ChangeRoleCommand>,
    c: web::Data<Container>,
) -> Result<HttpResponse, PublicError> {
    let user_id = auth(&req, &c).await?;

    ChangeRole::new(c.identity.role_repo(), c.identity.user_repo())
        .exec(user_id, path.into_inner(), cmd.into_inner())
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
                .route("", web::get().to(get_all))
                .route("/{user_id}", web::get().to(get_by_id))
                .route("/{user_id}", web::put().to(update))
                .route("/{user_id}", web::delete().to(delete))
                .route("/{user_id}/password", web::put().to(change_password))
                .route(
                    "/{user_id}/validate/{validation_code}",
                    web::get().to(validate),
                )
                .route("/{user_id}/role}", web::put().to(change_role)),
        );
}
