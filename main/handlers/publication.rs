use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use publishing::application::publication::{
    AddReview, AddReviewCommand, Approve, Create, CreateCommand, Delete, DeleteReview, GetById,
    Like, Publish, Read, Reject, Reviews, Search, SearchCommand, Unlike, Update, UpdateCommand,
    UpdatePages, UpdatePagesCommand,
};

use crate::authorization::with_auth;
use crate::container::{with_container, Container};
use crate::response;

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /publications/:id
    let get_by_id = warp::get()
        .and(warp::path!(String))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(get_by_id);

    // GET /publications
    let search = warp::get()
        .and(warp::path::end())
        .and(warp::query::<SearchCommand>())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(search);

    // POST /publications
    let create = warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(create);

    // PUT /publications/:id
    let update = warp::put()
        .and(warp::path!(String))
        .and(warp::body::json())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(update);

    // PUT /publications/:id/pages
    let update_pages = warp::put()
        .and(warp::path!(String / "pages"))
        .and(warp::body::json())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(update_pages);

    // DELETE /publications/:id
    let delete = warp::delete()
        .and(warp::path!(String))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(delete);

    // POST /publications/:id/publish
    let publish = warp::post()
        .and(warp::path!(String / "publish"))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(publish);

    // POST /publications/:id/approve
    let approve = warp::post()
        .and(warp::path!(String / "approve"))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(approve);

    // POST /publications/:id/reject
    let reject = warp::post()
        .and(warp::path!(String / "reject"))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(reject);

    // POST /publications/:id/read
    let read = warp::post()
        .and(warp::path!(String / "read"))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(read);

    // POST /publications/:id/like
    let like = warp::post()
        .and(warp::path!(String / "like"))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(like);

    // POST /publications/:id/unlike
    let unlike = warp::post()
        .and(warp::path!(String / "unlike"))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(unlike);

    // POST /publications/:id/review
    let review = warp::post()
        .and(warp::path!(String / "review"))
        .and(warp::body::json())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(review);

    // DELETE /publications/:id/review
    let delete_review = warp::delete()
        .and(warp::path!(String / "review"))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(delete_review);

    // GET /publications/:id/reviews
    let reviews = warp::get()
        .and(warp::path!(String / "reviews"))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(reviews);

    warp::path("publications").and(
        get_by_id
            .or(search)
            .or(create)
            .or(update)
            .or(update_pages)
            .or(delete)
            .or(publish)
            .or(approve)
            .or(reject)
            .or(read)
            .or(like)
            .or(unlike)
            .or(review)
            .or(reviews)
            .or(delete_review),
    )
}

pub async fn get_by_id(
    id: String,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let author_trans = c.author_translator();
    let uc = GetById::new(
        c.publishing.event_pub(),
        &author_trans,
        // c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    );
    let res = uc.exec(user_id, id).await;

    response::map(res, None)
}

pub async fn search(
    cmd: SearchCommand,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Search::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.content_manager_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(user_id, cmd).await;

    response::map(res, None)
}

pub async fn create(
    cmd: CreateCommand,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Create::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(user_id, cmd).await;

    response::map(res, None)
}

pub async fn update(
    id: String,
    cmd: UpdateCommand,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Update::new(
        c.publishing.event_pub(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(user_id, id, cmd).await;

    response::map(res, None)
}

pub async fn update_pages(
    id: String,
    cmd: UpdatePagesCommand,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = UpdatePages::new(c.publishing.event_pub(), c.publishing.publication_repo());
    let res = uc.exec(user_id, id, cmd).await;

    response::map(res, None)
}

pub async fn delete(
    id: String,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Delete::new(c.publishing.event_pub(), c.publishing.publication_repo());
    let res = uc.exec(user_id, id).await;

    response::map(res, None)
}

pub async fn publish(
    id: String,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Publish::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(user_id, id).await;

    response::map(res, None)
}

pub async fn approve(
    id: String,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Approve::new(
        c.publishing.event_pub(),
        c.publishing.content_manager_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(user_id, id).await;

    response::map(res, None)
}

pub async fn reject(
    id: String,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Reject::new(
        c.publishing.event_pub(),
        c.publishing.content_manager_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(user_id, id).await;

    response::map(res, None)
}

pub async fn read(id: String, user_id: String, c: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = Read::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    );
    let res = uc.exec(user_id, id).await;

    response::map(res, None)
}

pub async fn like(id: String, user_id: String, c: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = Like::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    );
    let res = uc.exec(user_id, id).await;

    response::map(res, None)
}

pub async fn unlike(
    id: String,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Unlike::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    );
    let res = uc.exec(user_id, id).await;

    response::map(res, None)
}

pub async fn review(
    id: String,
    cmd: AddReviewCommand,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = AddReview::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    );
    let res = uc.exec(user_id, id, cmd).await;

    response::map(res, None)
}

pub async fn delete_review(
    id: String,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = DeleteReview::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    );
    let res = uc.exec(user_id, id).await;

    response::map(res, None)
}

pub async fn reviews(
    id: String,
    _user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Reviews::new(c.publishing.interaction_repo(), c.publishing.reader_repo());
    let res = uc.exec(id).await;

    response::map(res, None)
}
