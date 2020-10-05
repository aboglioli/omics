use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use common::request::IncludeParams;
use publishing::application::author::{Follow, GetById, Search, SearchCommand, Unfollow};
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
async fn search(
    req: HttpRequest,
    cmd: web::Query<SearchCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    Search::new(c.publishing.author_repo())
        .exec(auth_id, cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[get("/{author_id}")]
async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    let mut user_id = path.into_inner();
    user_id = if user_id == "me" && auth_id.is_some() {
        auth_id.clone().unwrap()
    } else {
        user_id
    };

    GetById::new(c.publishing.author_repo(), c.publishing.interaction_repo())
        .exec(auth_id, user_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// TODO: consider other options of searching
#[get("/{author_id}/publications")]
async fn get_publications(
    req: HttpRequest,
    path: web::Path<String>,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    let mut user_id = path.into_inner();
    user_id = if user_id == "me" && auth_id.is_some() {
        auth_id.clone().unwrap()
    } else {
        user_id
    };

    SearchPublication::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
        c.publishing.user_repo(),
    )
    .exec(
        auth_id,
        SearchPublicationCommand {
            author_id: Some(user_id),
            category_id: None,
            tag: None,
            status: None,
            name: None,
        },
        include.into_inner().into(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[get("/{author_id}/collections")]
async fn get_collections(
    req: HttpRequest,
    path: web::Path<String>,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    let mut user_id = path.into_inner();
    user_id = if user_id == "me" && auth_id.is_some() {
        auth_id.clone().unwrap()
    } else {
        user_id
    };

    SearchCollection::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
    )
    .exec(
        auth_id,
        SearchCollectionCommand {
            author_id: Some(user_id),
            category_id: None,
            publication_id: None,
            tag: None,
            name: None,
        },
        include.into_inner().into(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[post("/{author_id}/follow")]
async fn follow(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Follow::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.interaction_repo(),
        c.publishing.reader_repo(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[post("/{author_id}/unfollow")]
async fn unfollow(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Unfollow::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
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
        web::scope("/authors")
            .service(search)
            .service(get_by_id)
            .service(get_publications)
            .service(get_collections)
            .service(follow)
            .service(unfollow),
    );
}
