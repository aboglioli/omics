use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use publishing::application::collection::{
    AddPublication, Create, CreateCommand, Delete, GetAll, GetById, RemovePublication, Update,
    UpdateCommand,
};

use crate::authorization::with_auth;
use crate::container::{with_container, Container};
use crate::response;

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /collections/:id
    let get_by_id = warp::get()
        .and(warp::path!(String))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(get_by_id);

    // GET /collections
    let get_all = warp::get()
        .and(warp::path::end())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(get_all);

    // POST /collections
    let create = warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(create);

    // PUT /collections/:id
    let update = warp::put()
        .and(warp::path!(String))
        .and(warp::body::json())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(update);

    // DELETE /collections/:id
    let delete = warp::delete()
        .and(warp::path!(String))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(delete);

    // POST /collections/:id/publication/:publication_id
    let add_publication = warp::post()
        .and(warp::path!(String / "publication" / String))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(add_publication);

    // POST /collections/:id/publication/:publication_id
    let remove_publication = warp::delete()
        .and(warp::path!(String / "publication" / String))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(remove_publication);

    warp::path("collections").and(
        get_by_id
            .or(get_all)
            .or(create)
            .or(update)
            .or(delete)
            .or(add_publication)
            .or(remove_publication),
    )
}

pub async fn get_by_id(
    id: String,
    _user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = GetById::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(id).await;

    response::map(res, None)
}

pub async fn get_all(_user_id: String, c: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = GetAll::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.collection_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec().await;

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
        c.publishing.collection_repo(),
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
        c.publishing.collection_repo(),
    );
    let res = uc.exec(user_id, id, cmd).await;

    response::map(res, None)
}

pub async fn delete(
    id: String,
    user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Delete::new(c.publishing.event_pub(), c.publishing.collection_repo());
    let res = uc.exec(user_id, id).await;

    response::map(res, None)
}

pub async fn add_publication(
    id: String,
    publication_id: String,
    _user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = AddPublication::new(
        c.publishing.event_pub(),
        c.publishing.collection_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(id, publication_id).await;

    response::map(res, None)
}

pub async fn remove_publication(
    id: String,
    publication_id: String,
    _user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = RemovePublication::new(c.publishing.event_pub(), c.publishing.collection_repo());
    let res = uc.exec(id, publication_id).await;

    response::map(res, None)
}
