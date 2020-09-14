use actix_web::{web, HttpRequest, HttpResponse, Responder};

use common::request::IncludeParams;
use publishing::application::publication::{
    AddReview, AddReviewCommand, Approve, ApproveCommand, Create, CreateCommand, Delete,
    DeleteReview, GetById, GetReviews, Like, Publish, Read, Reject, RejectCommand, Search,
    SearchCommand, Unlike, Update, UpdateCommand, UpdatePages, UpdatePagesCommand,
};
use publishing::application::collection::{Search as SearchCollection, SearchCommand as SearchCollectionCommand};

use crate::authorization::auth;
use crate::container::Container;
use crate::error::PublicError;

// POST /publications
async fn create(
    req: HttpRequest,
    cmd: web::Json<CreateCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Create::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
    )
    .exec(auth_id, cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// GET /publications
async fn search(
    req: HttpRequest,
    cmd: web::Query<SearchCommand>,
    include: web::Query<IncludeParams>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    Search::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
        c.publishing.user_repo(),
    )
    .exec(auth_id, cmd.into_inner(), include.into_inner().into())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// GET /publications/:id
async fn get_by_id(
    req: HttpRequest,
    path: web::Path<String>,
    include: web::Query<IncludeParams>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    GetById::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.user_repo(),
        c.publishing.interaction_serv(),
        c.publishing.statistics_serv(),
    )
    .exec(auth_id, path.into_inner(), include.into_inner().into())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// PUT /publications/:id
async fn update(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<UpdateCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Update::new(
        c.publishing.event_pub(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
    )
    .exec(auth_id, path.into_inner(), cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// PUT /publications/:id/pages
async fn update_pages(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<UpdatePagesCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    UpdatePages::new(c.publishing.event_pub(), c.publishing.publication_repo())
        .exec(auth_id, path.into_inner(), cmd.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// DELETE /publications/:id
async fn delete(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Delete::new(c.publishing.event_pub(), c.publishing.publication_repo())
        .exec(auth_id, path.into_inner())
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(PublicError::from)
}

// POST /publications/:id/publish
async fn publish(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Publish::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.publication_repo(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// POST /publications/:id/approve
async fn approve(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<ApproveCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Approve::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.user_repo(),
    )
    .exec(auth_id, path.into_inner(), cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// POST /publications/:id/reject
async fn reject(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<RejectCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Reject::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.user_repo(),
    )
    .exec(auth_id, path.into_inner(), cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// GET /publications/:id/read
async fn read(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Read::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// POST /publications/:id/like
async fn like(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Like::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// POST /publications/:id/unlike
async fn unlike(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    Unlike::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// POST /publications/:id/review
async fn review(
    req: HttpRequest,
    path: web::Path<String>,
    cmd: web::Json<AddReviewCommand>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    AddReview::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    )
    .exec(auth_id, path.into_inner(), cmd.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// DELETE /publications/:id/review
async fn delete_review(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await?;

    DeleteReview::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// GET /publications/:id/reviews
async fn get_reviews(
    req: HttpRequest,
    path: web::Path<String>,
    c: web::Data<Container>,
) -> impl Responder {
    let auth_id = auth(&req, &c).await.ok();

    GetReviews::new(
        c.publishing.interaction_repo(),
        c.publishing.reader_repo(),
        c.publishing.user_repo(),
    )
    .exec(auth_id, path.into_inner())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

// GET /publications/:id/collections
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
    .exec(auth_id, SearchCollectionCommand {
        author_id: None,
        category_id: None,
        publication_id: Some(path.into_inner()),
        name: None,
    }, include.into_inner().into())
    .await
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(PublicError::from)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/publications")
            .route("", web::post().to(create))
            .route("", web::get().to(search))
            .route("/{publicaton_id}", web::get().to(get_by_id))
            .route("/{publicaton_id}", web::put().to(update))
            .route("/{publicaton_id}/pages", web::put().to(update_pages))
            .route("/{publicaton_id}", web::delete().to(delete))
            .route("/{publicaton_id}/publish", web::post().to(publish))
            .route("/{publicaton_id}/approve", web::post().to(approve))
            .route("/{publicaton_id}/reject", web::post().to(reject))
            .route("/{publicaton_id}/read", web::get().to(read))
            .route("/{publicaton_id}/like", web::post().to(like))
            .route("/{publicaton_id}/unlike", web::post().to(unlike))
            .route("/{publicaton_id}/review", web::post().to(review))
            .route("/{publicaton_id}/review", web::delete().to(delete_review))
            .route("/{publicaton_id}/reviews", web::get().to(get_reviews))
            .route("/{publicaton_id}/collections", web::get().to(get_collections)),
    );
}
