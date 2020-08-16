use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use publishing::application::publication::{
    AddReview, AddReviewCommand, Approve, Create, CreateCommand, Delete, GetById, Like, Publish,
    Read, Reject, Reviews, Search, SearchCommand, Unlike, Update, UpdateCommand, UpdatePages,
    UpdatePagesCommand,
};

use crate::authorization;
use crate::container::{with_container, Container};
use crate::response;

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /publications/:id
    let get_by_id = warp::get()
        .and(warp::path!(String))
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(get_by_id);

    // GET /publications
    let _search = warp::get()
        .and(warp::path::end())
        .and(warp::query::<SearchCommand>())
        .and(with_container(container.clone()))
        .and_then(search);

    // POST /publications
    let create = warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(create);

    // PUT /publications/:id
    let update = warp::put()
        .and(warp::path!(String))
        .and(warp::body::json())
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(update);

    // PUT /publications/:id/pages
    let _update_pages = warp::put()
        .and(warp::path!(String / "pages"))
        .and(warp::body::json())
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(update_pages);

    // DELETE /publications/:id
    let delete = warp::delete()
        .and(warp::path!(String))
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(delete);

    // POST /publications/:id/publish
    let publish = warp::post()
        .and(warp::path!(String / "publish"))
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(publish);

    // POST /publications/:id/approve
    let approve = warp::post()
        .and(warp::path!(String / "approve"))
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(approve);

    // POST /publications/:id/reject
    let reject = warp::post()
        .and(warp::path!(String / "reject"))
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(reject);

    // POST /publications/:id/read
    let read = warp::post()
        .and(warp::path!(String / "read"))
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(read);

    // POST /publications/:id/like
    let like = warp::post()
        .and(warp::path!(String / "like"))
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(like);

    // POST /publications/:id/unlike
    let _unlike = warp::post()
        .and(warp::path!(String / "unlike"))
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(unlike);

    // POST /publications/:id/review
    let review = warp::post()
        .and(warp::path!(String / "review"))
        .and(warp::body::json())
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(review);

    // GET /publications/:id/reviesws
    let _reviews = warp::get()
        .and(warp::path!(String / "reviews"))
        .and(warp::header::<String>("authorization"))
        .and(with_container(container.clone()))
        .and_then(reviews);

    warp::path("publications").and(
        get_by_id
            .or(create)
            .or(update)
            .or(delete)
            .or(publish)
            .or(approve)
            .or(reject)
            .or(read)
            .or(like)
            .or(review),
    )
}

pub async fn get_by_id(
    id: String,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let user_id = authorization::with_user(&authorization_header, &c)
        .await
        .unwrap();

    let uc = GetById::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    );
    let res = uc.exec(user_id, id).await;

    response::check(res, None)
}

pub async fn search(cmd: SearchCommand, c: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = Search::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(cmd).await;

    response::check(res, None)
}

pub async fn create(
    cmd: CreateCommand,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let user_id = authorization::with_user(&authorization_header, &c)
        .await
        .unwrap();

    let uc = Create::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(user_id, cmd).await;

    response::check(res, None)
}

pub async fn update(
    id: String,
    cmd: UpdateCommand,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let user_id = authorization::with_user(&authorization_header, &c)
        .await
        .unwrap();

    let uc = Update::new(
        c.publishing.event_pub(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(user_id, id, cmd).await;

    response::check(res, None)
}

pub async fn update_pages(
    id: String,
    cmd: UpdatePagesCommand,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let user_id = authorization::with_user(&authorization_header, &c)
        .await
        .unwrap();

    let uc = UpdatePages::new(c.publishing.event_pub(), c.publishing.publication_repo());
    let res = uc.exec(user_id, id, cmd).await;

    response::check(res, None)
}

pub async fn delete(
    id: String,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let user_id = authorization::with_user(&authorization_header, &c)
        .await
        .unwrap();

    let uc = Delete::new(c.publishing.event_pub(), c.publishing.publication_repo());
    let res = uc.exec(user_id, id).await;

    response::check(res, None)
}

pub async fn publish(
    id: String,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let user_id = authorization::with_user(&authorization_header, &c)
        .await
        .unwrap();

    let uc = Publish::new(
        c.publishing.event_pub(),
        c.publishing.author_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(user_id, id).await;

    response::check(res, None)
}

pub async fn approve(
    id: String,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let user_id = authorization::with_user(&authorization_header, &c)
        .await
        .unwrap();

    let uc = Approve::new(
        c.publishing.event_pub(),
        c.publishing.content_manager_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(user_id, id).await;

    response::check(res, None)
}

pub async fn reject(
    id: String,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let user_id = authorization::with_user(&authorization_header, &c)
        .await
        .unwrap();

    let uc = Reject::new(
        c.publishing.event_pub(),
        c.publishing.content_manager_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(user_id, id).await;

    response::check(res, None)
}

pub async fn read(
    id: String,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let user_id = authorization::with_user(&authorization_header, &c)
        .await
        .unwrap();

    let uc = Read::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    );
    let res = uc.exec(user_id, id).await;

    response::check(res, None)
}

pub async fn like(
    id: String,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let user_id = authorization::with_user(&authorization_header, &c)
        .await
        .unwrap();

    let uc = Like::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    );
    let res = uc.exec(user_id, id).await;

    response::check(res, None)
}

pub async fn unlike(
    id: String,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let user_id = authorization::with_user(&authorization_header, &c)
        .await
        .unwrap();

    let uc = Unlike::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    );
    let res = uc.exec(user_id, id).await;

    response::check(res, None)
}

pub async fn review(
    id: String,
    cmd: AddReviewCommand,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let user_id = authorization::with_user(&authorization_header, &c)
        .await
        .unwrap();

    let uc = AddReview::new(
        c.publishing.event_pub(),
        c.publishing.publication_repo(),
        c.publishing.reader_repo(),
        c.publishing.interaction_serv(),
    );
    let res = uc.exec(user_id, id, cmd).await;

    response::check(res, None)
}

pub async fn reviews(
    id: String,
    authorization_header: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let _user_id = authorization::with_user(&authorization_header, &c)
        .await
        .unwrap();

    let uc = Reviews::new(c.publishing.interaction_repo());
    let res = uc.exec(id).await;

    response::check(res, None)
}
