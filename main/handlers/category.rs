use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};

use common::request::{IncludeParams, PaginationParams};
use publishing::application::category::{
    Create, CreateCommand, Delete, GetAll, GetById, Update, UpdateCommand,
};
use publishing::application::collection::{
    Search as SearchCollection, SearchCommand as SearchCollectionCommand,
};
use publishing::application::publication::{
    Search as SearchPublication, SearchCommand as SearchPublicationCommand,
};

use crate::authorization::auth;
use crate::container::MainContainer;
use crate::error::PublicError;

#[get("")]
async fn get_all(req: HttpRequest, c: web::Data<MainContainer>) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await.ok();

    GetAll::new(c.publishing.category_repo())
        .exec(user_id_and_role)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[get("/{category_id}")]
async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await.ok();

    GetById::new(c.publishing.category_repo())
        .exec(user_id_and_role, path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[get("/{category_id}/publications")]
async fn get_publications(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Query<SearchPublicationCommand>,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await.ok();

    let mut cmd = cmd.into_inner();
    cmd.category_id = Some(path.into_inner());

    SearchPublication::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
        c.publishing.user_repo(),
    )
    .exec(
        user_id_and_role,
        cmd,
        include.into_inner().into(),
        PaginationParams::default(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[get("/{category_id}/collections")]
async fn get_collections(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Query<SearchCollectionCommand>,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await.ok();

    let mut cmd = cmd.into_inner();
    cmd.category_id = Some(path.into_inner());

    SearchCollection::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
    )
    .exec(
        user_id_and_role,
        cmd,
        include.into_inner().into(),
        PaginationParams::default(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[post("")]
async fn create(
    req: HttpRequest,
    cmd: web::Json<CreateCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    Create::new(c.publishing.event_pub(), c.publishing.category_repo())
        .exec(user_id_and_role, cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[put("/{category_id}")]
async fn update(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<UpdateCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    Update::new(c.publishing.event_pub(), c.publishing.category_repo())
        .exec(user_id_and_role, path.into_inner(), cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[delete("/{category_id}")]
async fn delete(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    Delete::new(c.publishing.event_pub(), c.publishing.category_repo())
        .exec(user_id_and_role, path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .service(get_all)
            .service(get_by_id)
            .service(get_publications)
            .service(get_collections)
            .service(create)
            .service(update)
            .service(delete),
    );
}
