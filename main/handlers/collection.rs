use actix_web::{web, HttpRequest, HttpResponse, Responder};

use publishing::application::collection::{
    AddPublication, Create, CreateCommand, Delete, GetAll, GetById, RemovePublication, Update,
    UpdateCommand,
};

use crate::authorization::auth;
use crate::container::Container;
use crate::error::PublicError;

async fn create(
    req: HttpRequest,
    cmd: web::Json<CreateCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let user_id = auth(&req, &c).await?;

    Create::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
    )
    .exec(user_id, cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

async fn get_all(req: HttpRequest, c: web::Data<Container>) -> impl Responder {
    let _user_id = auth(&req, &c).await?;

    GetAll::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
        c.publishing.publication_repo(),
    )
    .exec()
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let _user_id = auth(&req, &c).await?;

    GetById::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
        c.publishing.publication_repo(),
    )
    .exec(path.into_inner())
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
    let user_id = auth(&req, &c).await?;

    Update::new(
        c.publishing.event_pub(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
    )
    .exec(user_id, path.into_inner(), cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

async fn delete(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let user_id = auth(&req, &c).await?;

    Delete::new(c.publishing.event_pub(), c.publishing.collection_repo())
        .exec(user_id, path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

async fn add_publication(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    c: web::Data<Container>,
) -> impl Responder {
    let _user_id = auth(&req, &c).await?;

    let path = path.into_inner();
    AddPublication::new(
        c.publishing.event_pub(),
        c.publishing.collection_repo(),
        c.publishing.publication_repo(),
    )
    .exec(path.0, path.1)
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

async fn remove_publication(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    c: web::Data<Container>,
) -> impl Responder {
    let _user_id = auth(&req, &c).await?;

    let path = path.into_inner();
    RemovePublication::new(c.publishing.event_pub(), c.publishing.collection_repo())
        .exec(path.0, path.1)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/collections")
            .route("", web::post().to(create))
            .route("", web::get().to(get_all))
            .route("/{collection_id}", web::get().to(get_by_id))
            .route("/{collection_id}", web::put().to(update))
            .route("/{collection_id}", web::delete().to(delete))
            .route(
                "/{collection_id}/publication/{publication_id}",
                web::post().to(add_publication),
            )
            .route(
                "/{collection_id}/publication/{publication_id}",
                web::delete().to(remove_publication),
            ),
    );
}
