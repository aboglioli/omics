use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use common::request::{IncludeParams, PaginationParams};
use payment::application::donation::{
    Donate, DonateCommand, Search as SearchDonation, SearchCommand as SearchDonationCommand,
};
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
    pagination: web::Query<PaginationParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await.ok();

    Search::new(c.publishing.author_repo())
        .exec(user_id_and_role, cmd.into_inner(), pagination.into_inner())
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
    let user_id_and_role = auth(&req, &c).await.ok();

    let mut user_id = path.into_inner();
    if user_id == "me" {
        if let Some((auth_id, _)) = &user_id_and_role {
            user_id = auth_id.to_string();
        }
    }

    GetById::new(c.publishing.author_repo(), c.publishing.interaction_repo())
        .exec(user_id_and_role, user_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

#[get("/{author_id}/publications")]
async fn get_publications(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Query<SearchPublicationCommand>,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await.ok();

    let mut user_id = path.into_inner();
    if user_id == "me" {
        if let Some((auth_id, _)) = &user_id_and_role {
            user_id = auth_id.to_string();
        }
    }

    let mut cmd = cmd.into_inner();
    cmd.author_id = Some(user_id);

    SearchPublication::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
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

#[get("/{author_id}/collections")]
async fn get_collections(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Query<SearchCollectionCommand>,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await.ok();

    let mut user_id = path.into_inner();
    if user_id == "me" {
        if let Some((auth_id, _)) = &user_id_and_role {
            user_id = auth_id.to_string();
        }
    }

    let mut cmd = cmd.into_inner();
    cmd.author_id = Some(user_id);

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

#[post("/{author_id}/follow")]
async fn follow(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    Follow::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.interaction_repo(),
        c.publishing.reader_repo(),
    )
    .exec(user_id_and_role, path.into_inner())
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
    let user_id_and_role = auth(&req, &c).await?;

    Unfollow::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.interaction_repo(),
        c.publishing.reader_repo(),
    )
    .exec(user_id_and_role, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

#[get("/{author_id}/donations")]
async fn get_donations(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Query<SearchDonationCommand>,
    include: web::Query<IncludeParams>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    let mut user_id = path.into_inner();
    if user_id == "me" {
        user_id = user_id_and_role.0.to_string();
    }

    let mut cmd = cmd.into_inner();
    cmd.author_id = Some(user_id);

    SearchDonation::new(
        c.publishing.author_repo(),
        c.payment.donation_repo(),
        c.publishing.reader_repo(),
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

#[post("/{author_id}/donate")]
async fn donate(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<DonateCommand>,
    c: web::Data<MainContainer>,
) -> impl Responder {
    let user_id_and_role = auth(&req, &c).await?;

    Donate::new(
        c.payment.event_pub(),
        c.publishing.author_repo(),
        c.payment.donation_repo(),
        c.publishing.reader_repo(),
        c.identity.user_repo(),
        c.config_serv(),
        c.payment.payment_serv(),
    )
    .exec(user_id_and_role, path.into_inner(), cmd.into_inner())
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
            .service(get_donations)
            .service(follow)
            .service(unfollow)
            .service(donate),
    );
}
