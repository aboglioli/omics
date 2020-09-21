use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};

use common::request::IncludeParams;
use publishing::application::collection::{
    AddPublication, AddToFavorites, Create, CreateCommand, Delete, GetById, GetPublications,
    RemoveFromFavorites, RemovePublication, Search, SearchCommand, Update, UpdateCommand,
};

use crate::authorization::auth;
use crate::container::MainContainer;
use crate::error::PublicError;

#[post("")]
async fn create(
    req: HttpRequest,
    cmd: web::Json<CreateCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Create::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
    )
    .exec(auth_id, cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[get("")]
async fn search(
    req: HttpRequest,
    cmd: web::Query<SearchCommand>,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    Search::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
    )
    .exec(auth_id, cmd.into_inner(), include.into_inner().into())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[get("/{collection_id}")]
async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    GetById::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
    )
    .exec(auth_id, path.into_inner(), include.into_inner().into())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[get("/{collection_id}/publications")]
async fn get_publications(
    req: HttpRequest,
    path: web::Path<String>,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    GetPublications::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
        c.publishing.publication_repo(),
        c.publishing.user_repo(),
    )
    .exec(auth_id, path.into_inner(), include.into_inner().into())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[put("/{collection_id}")]
async fn update(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<UpdateCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Update::new(
        c.publishing.event_pub(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
    )
    .exec(auth_id, path.into_inner(), cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[delete("/{collection_id}")]
async fn delete(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Delete::new(c.publishing.event_pub(), c.publishing.collection_repo())
        .exec(auth_id, path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[post("/{collection_id}/publication/{publication_id}")]
async fn add_publication(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    let path = path.into_inner();
    AddPublication::new(
        c.publishing.event_pub(),
        c.publishing.collection_repo(),
        c.publishing.publication_repo(),
    )
    .exec(auth_id, path.0, path.1)
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[delete("/{collection_id}/publication/{publication_id}")]
async fn remove_publication(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    let path = path.into_inner();
    RemovePublication::new(c.publishing.event_pub(), c.publishing.collection_repo())
        .exec(auth_id, path.0, path.1)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[post("/{collection_id}/favorite")]
async fn add_to_favorites(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    AddToFavorites::new(
        c.publishing.event_pub(),
        c.publishing.collection_repo(),
        c.publishing.interaction_repo(),
        c.publishing.reader_repo(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[delete("/{collection_id}/favorite")]
async fn remove_from_favorites(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    RemoveFromFavorites::new(
        c.publishing.event_pub(),
        c.publishing.collection_repo(),
        c.publishing.interaction_repo(),
        c.publishing.reader_repo(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/collections")
            .service(create)
            .service(search)
            .service(get_by_id)
            .service(get_publications)
            .service(update)
            .service(delete)
            .service(add_publication)
            .service(remove_publication)
            .service(add_to_favorites)
            .service(remove_from_favorites),
    );
}
