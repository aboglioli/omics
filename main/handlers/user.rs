use actix_web::{delete, get, http, post, put, web, HttpRequest, HttpResponse, Responder};

use common::request::{IncludeParams, PaginationParams};
use identity::application::user::{
    ChangePassword, ChangePasswordCommand, ChangePaymentEmail, ChangePaymentEmailCommand,
    ChangeRole, ChangeRoleCommand, Delete, GetById, Login, LoginCommand, RecoverPassword,
    RecoverPasswordCommand, Register, RegisterCommand, Search, SearchCommand, Update,
    UpdateCommand, Validate,
};

use crate::authorization::auth;
use crate::container::MainContainer;
use crate::error::PublicError;

#[post("/register")]
async fn register(cmd: web::Json<RegisterCommand>, c: web::Data<MainContainer>) -> impl Responder {
    Register::new(
        c.identity.event_pub(),
        c.identity.role_repo(),
        c.identity.user_repo(),
        c.identity.user_serv(),
    )
    .exec(cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[post("/login")]
async fn login(cmd: web::Json<LoginCommand>, c: web::Data<MainContainer>) -> impl Responder {
    Login::new(
        c.identity.event_pub(),
        c.identity.role_repo(),
        c.identity.authentication_serv(),
    )
    .exec(cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[post("/recover-password")]
async fn recover_password(
    cmd: web::Json<RecoverPasswordCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    RecoverPassword::new(
        c.identity.event_pub(),
        c.identity.role_repo(),
        c.identity.user_repo(),
        c.identity.user_serv(),
    )
    .exec(cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[get("")]
async fn search(
    req: HttpRequest,
    cmd: web::Query<SearchCommand>,
    include: web::Query<IncludeParams>,
    pagination: web::Query<PaginationParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    Search::new(c.identity.role_repo(), c.identity.user_repo())
        .exec(
            user_id_and_role,
            cmd.into_inner(),
            include.into_inner().into(),
            pagination.into_inner(),
        )
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[get("/{user_id}")]
async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    let mut user_id = path.into_inner();
    if user_id == "me" {
        if let (auth_id, _) = &user_id_and_role {
            user_id = auth_id.to_string();
        }
    }

    GetById::new(c.identity.role_repo(), c.identity.user_repo())
        .exec(user_id_and_role, user_id, include.into_inner().into())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[put("/{user_id}")]
async fn update(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<UpdateCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    let mut user_id = path.into_inner();
    if user_id == "me" {
        if let (auth_id, _) = &user_id_and_role {
            user_id = auth_id.to_string();
        }
    }

    Update::new(c.identity.event_pub(), c.identity.user_repo())
        .exec(user_id_and_role, user_id, cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[delete("/{user_id}")]
async fn delete(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    let mut user_id = path.into_inner();
    if user_id == "me" {
        if let (auth_id, _) = &user_id_and_role {
            user_id = auth_id.to_string();
        }
    }

    Delete::new(c.identity.event_pub(), c.identity.user_repo())
        .exec(user_id_and_role, user_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[put("/{user_id}/password")]
async fn change_password(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<ChangePasswordCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await.ok();

    let mut user_id = path.into_inner();
    if user_id == "me" {
        if let Some((auth_id, _)) = &user_id_and_role {
            user_id = auth_id.to_string();
        }
    }

    ChangePassword::new(
        c.identity.event_pub(),
        c.identity.role_repo(),
        c.identity.user_repo(),
        c.identity.user_serv(),
    )
    .exec(user_id, cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[get("/{user_id}/validate/{code}")]
async fn validate(
    path: web::Path<(String, String)>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let path = path.into_inner();

    Validate::new(
        c.identity.event_pub(),
        c.identity.role_repo(),
        c.identity.user_repo(),
    )
    .exec(path.0, path.1)
    .await
    .map(|_res| {
        HttpResponse::Ok()
            .header(http::header::LOCATION, "http://localhost:4200")
            .content_type("text/html")
            .body(
                r#"
                Bienvenido. Tu cuenta ha sido verificada.
                <a href="http://localhost:4200/">Continua</a>.
            "#,
            )
    })
    .map_err(PublicError::from)
}

#[put("/{user_id}/payment-email")]
async fn change_payment_email(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<ChangePaymentEmailCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    let mut user_id = path.into_inner();
    if user_id == "me" {
        if let (auth_id, _) = &user_id_and_role {
            user_id = auth_id.to_string();
        }
    }

    ChangePaymentEmail::new(c.identity.event_pub(), c.identity.user_repo())
        .exec(user_id_and_role, user_id, cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[put("/{user_id}/role")]
async fn change_role(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<ChangeRoleCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    let mut user_id = path.into_inner();
    if user_id == "me" {
        if let (auth_id, _) = &user_id_and_role {
            user_id = auth_id.to_string();
        }
    }

    ChangeRole::new(
        c.identity.event_pub(),
        c.identity.role_repo(),
        c.identity.user_repo(),
    )
    .exec(user_id_and_role, user_id, cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register)
        .service(login)
        .service(recover_password)
        .service(
            web::scope("/users")
                .service(search)
                .service(get_by_id)
                .service(update)
                .service(delete)
                .service(change_password)
                .service(validate)
                .service(change_role)
                .service(change_payment_email),
        );
}
