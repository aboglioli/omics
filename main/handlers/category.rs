use actix_web::{web, HttpRequest, HttpResponse, Responder};

use common::request::IncludeParams;
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
use crate::container::Container;
use crate::error::PublicError;

// GET /categories
async fn get_all(req: HttpRequest, c: web::Data<Container>) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    GetAll::new(c.publishing.category_repo())
        .exec(auth_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// GET /categories/:id
async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    GetById::new(c.publishing.category_repo())
        .exec(auth_id, path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

async fn get_publications(
    req: HttpRequest,
    path: web::Path<String>,
    include: web::Query<IncludeParams>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    SearchPublication::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
        c.publishing.user_repo(),
    )
    .exec(
        auth_id,
        SearchPublicationCommand {
            author_id: None,
            category_id: Some(path.into_inner()),
            status: None,
            name: None,
        },
        include.into_inner().into(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

async fn get_collections(
    req: HttpRequest,
    path: web::Path<String>,
    include: web::Query<IncludeParams>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    SearchCollection::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
        c.publishing.user_repo(),
    )
    .exec(
        auth_id,
        SearchCollectionCommand {
            author_id: None,
            category_id: Some(path.into_inner()),
            publication_id: None,
            name: None,
        },
        include.into_inner().into(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

async fn create(
    req: HttpRequest,
    cmd: web::Json<CreateCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Create::new(c.publishing.category_repo(), c.publishing.user_repo())
        .exec(auth_id, cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

async fn update(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<UpdateCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Update::new(c.publishing.category_repo(), c.publishing.user_repo())
        .exec(auth_id, path.into_inner(), cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

async fn delete(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Delete::new(c.publishing.category_repo(), c.publishing.user_repo())
        .exec(auth_id, path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .route("", web::get().to(get_all))
            .route("/{category_id}", web::get().to(get_by_id))
            .route(
                "/{category_id}/publications",
                web::get().to(get_publications),
            )
            .route("/{category_id}/collections", web::get().to(get_collections))
            .route("", web::post().to(create))
            .route("/{category_id}", web::put().to(update))
            .route("/{category_id}", web::delete().to(delete)),
    );
}
