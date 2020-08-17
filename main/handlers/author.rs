use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use publishing::application::author::{GetById, Search, SearchCommand};

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
    let search = warp::get()
        .and(warp::path::end())
        .and(warp::query::<SearchCommand>())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(search);

    warp::path("authors").and(get_by_id.or(search))
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

pub async fn search(
    cmd: SearchCommand,
    _user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = Search::new(
        c.publishing.author_repo(),
        c.publishing.collection_repo(),
        c.publishing.publication_repo(),
    );
    let res = uc.exec(cmd).await;

    response::map(res, None)
}
