use actix_web::{web, HttpRequest, HttpResponse, Responder};

use common::request::IncludeParams;
use publishing::application::author::{Follow, GetById, Search, SearchCommand};
use publishing::application::collection::{
    Search as SearchCollection, SearchCommand as SearchCollectionCommand,
};
use publishing::application::publication::{
    Search as SearchPublication, SearchCommand as SearchPublicationCommand,
};

use crate::authorization::auth;
use crate::container::Container;
use crate::error::PublicError;

// GET /authors
async fn search(
    req: HttpRequest,
    cmd: web::Query<SearchCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    Search::new(c.publishing.author_repo())
        .exec(auth_id, cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// GET /authors/:id
async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    GetById::new(c.publishing.author_repo())
        .exec(auth_id, path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// GET /authors/:id/publications
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
        c.publishing.content_manager_repo(),
        c.publishing.publication_repo(),
    )
    .exec(
        auth_id,
        SearchPublicationCommand {
            author_id: Some(path.into_inner()),
            category_id: None,
            status: None,
            name: None,
        },
        include.into_inner().into(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// GET /authors/:id/collections
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
    )
    .exec(
        auth_id,
        SearchCollectionCommand {
            author_id: Some(path.into_inner()),
            category_id: None,
            publication_id: None,
            name: None,
        },
        include.into_inner().into(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// POST /authors/:id/follow
async fn follow(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Follow::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/authors")
            .route("", web::get().to(search))
            .route("/{author_id}", web::get().to(get_by_id))
            .route("/{author_id}/publications", web::get().to(get_publications))
            .route("/{author_id}/collections", web::get().to(get_collections))
            .route("/{author_id}/follow", web::post().to(follow)),
    );
}
