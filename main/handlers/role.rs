use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use common::request::{IncludeParams, PaginationParams};
use identity::application::role::{GetAll, GetById};
use identity::application::user::{Search as SearchUser, SearchCommand as SearchUserCommand};

use crate::authorization::auth;
use crate::container::MainContainer;
use crate::error::PublicError;

#[get("")]
async fn get_all(req: HttpRequest, c: web::Data<MainContainer>) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    GetAll::new(c.identity.role_repo(), c.identity.user_repo())
        .exec(user_id_and_role)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[get("/{role_id}")]
async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    _include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    GetById::new(c.identity.role_repo(), c.identity.user_repo())
        .exec(user_id_and_role, path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[get("/{role_id}/users")]
async fn get_users(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Query<SearchUserCommand>,
    include: web::Query<IncludeParams>,
    pagination: web::Query<PaginationParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    let mut cmd = cmd.into_inner();
    cmd.role_id = Some(path.into_inner());

    SearchUser::new(c.identity.role_repo(), c.identity.user_repo())
        .exec(
            user_id_and_role,
            cmd,
            include.into_inner().into(),
            pagination.into_inner(),
        )
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/roles")
            .service(get_all)
            .service(get_by_id)
            .service(get_users),
    );
}
