use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};

use identity::application::user::{
    ChangePassword, ChangePasswordCommand, ChangeRole, ChangeRoleCommand, Delete, GetAll, GetById,
    Login, LoginCommand, RecoverPassword, RecoverPasswordCommand, Register, RegisterCommand,
    RegisterResponse, Update, UpdateCommand, Validate,
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

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        // .route("/recover-password", web::post().to(recover_password))
        .service(
            web::scope("/users")
                .route("", web::get().to(get_all))
                .route("/{user_id}", web::get().to(get_by_id)), // .route("/{user_id}", web::put().to(update))
                                                                // .route("/{user_id}", web::delete().to(delete))
                                                                // .route("/{user_id}/password", web::put().to(change_password))
                                                                // .route("/{user_id}/validate/{validation_code}", web::get().to(validate))
                                                                // .route("/{user_id}/role}", web::put().to(change_role))
        );
}

// pub async fn get_all(user_id: String, c: Arc<Container>) -> Result<impl Reply, Rejection> {
//     let uc = GetAll::new(c.identity.user_repo());
//     let res = uc.exec(user_id).await;
//
//     response::map(res, None)
// }
//
// pub async fn get_by_id(
//     id: String,
//     user_id: String,
//     c: Arc<Container>,
// ) -> Result<impl Reply, Rejection> {
//     let uc = GetById::new(c.identity.user_repo());
//     let res = uc.exec(user_id, id).await;
//
//     response::map(res, None)
// }
//
// pub async fn register(cmd: RegisterCommand, c: Arc<Container>) -> Result<impl Reply, Rejection> {
//     let uc = Register::new(
//         c.identity.event_pub(),
//         c.identity.user_repo(),
//         c.identity.user_serv(),
//     );
//     let res = uc.exec(cmd).await;
//
//     response::map(res, Some(StatusCode::CREATED))
// }
//
// pub async fn login(cmd: LoginCommand, c: Arc<Container>) -> Result<impl Reply, Rejection> {
//     let uc = Login::new(c.identity.event_pub(), c.identity.authentication_serv());
//     let res = uc.exec(cmd).await;
//
//     response::map(res, None)
// }
//
// pub async fn update(
//     id: String,
//     cmd: UpdateCommand,
//     _user_id: String,
//     c: Arc<Container>,
// ) -> Result<impl Reply, Rejection> {
//     let uc = Update::new(c.identity.event_pub(), c.identity.user_repo());
//     let res = uc.exec(id, cmd).await;
//
//     response::map(res, None)
// }
//
// pub async fn delete(
//     id: String,
//     _user_id: String,
//     c: Arc<Container>,
// ) -> Result<impl Reply, Rejection> {
//     let uc = Delete::new(c.identity.event_pub(), c.identity.user_repo());
//     let res = uc.exec(id).await;
//
//     response::map(res, None)
// }
//
// pub async fn change_password(
//     id: String,
//     cmd: ChangePasswordCommand,
//     _user_id: String,
//     c: Arc<Container>,
// ) -> Result<impl Reply, Rejection> {
//     let uc = ChangePassword::new(c.identity.user_serv());
//     let res = uc.exec(id, cmd).await;
//
//     response::map(res, None)
// }
//
// pub async fn recover_password(
//     cmd: RecoverPasswordCommand,
//     c: Arc<Container>,
// ) -> Result<impl Reply, Rejection> {
//     let uc = RecoverPassword::new(
//         c.identity.event_pub(),
//         c.identity.user_repo(),
//         c.identity.user_serv(),
//     );
//     let res = uc.exec(cmd).await;
//
//     response::map(res, None)
// }
//
// pub async fn validate(
//     id: String,
//     code: String,
//     c: Arc<Container>,
// ) -> Result<impl Reply, Rejection> {
//     let uc = Validate::new(c.identity.event_pub(), c.identity.user_repo());
//     let res = uc.exec(id, code).await;
//
//     response::map(res, None)
// }
//
// pub async fn change_role(
//     id: String,
//     cmd: ChangeRoleCommand,
//     user_id: String,
//     c: Arc<Container>,
// ) -> Result<impl Reply, Rejection> {
//     let uc = ChangeRole::new(c.identity.role_repo(), c.identity.user_repo());
//     let res = uc.exec(user_id, id, cmd).await;
//
//     response::map(res, None)
// }
