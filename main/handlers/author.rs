use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use publishing::application::author::{GetAll, GetById};

use crate::authorization::with_auth;
use crate::container::{with_container, Container};
use crate::response;

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /authors/:id
    let get_by_id = warp::get()
        .and(warp::path!(String))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(get_by_id);

    // GET /authors
    let get_all = warp::get()
        .and(warp::path::end())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(get_all);

    warp::path("authors").and(get_by_id.or(get_all))
}

pub async fn get_by_id(
    id: String,
    _user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = GetById::new(
        c.publishing.author_repo(),
        c.publishing.category_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(id).await;

    response::map(res, None)
}

pub async fn get_all(_user_id: String, c: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = GetAll::new(
        c.publishing.author_repo(),
        c.publishing.collection_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec().await;

    response::map(res, None)
}
