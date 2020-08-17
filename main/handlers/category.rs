use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use publishing::application::category::{GetAll, GetById};

use crate::container::{with_container, Container};
use crate::response;

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /categories
    let get_all = warp::get()
        .and(warp::path::end())
        .and(with_container(container.clone()))
        .and_then(get_all);

    // GET /categories/:id
    let get_by_id = warp::get()
        .and(warp::path!(String))
        .and(with_container(container.clone()))
        .and_then(get_by_id);

    warp::path("categories").and(get_all.or(get_by_id))
}

pub async fn get_by_id(id: String, c: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = GetById::new(c.publishing.category_repo());
    let res = uc.exec(id).await;

    response::map(res, None)
}

pub async fn get_all(c: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = GetAll::new(c.publishing.category_repo());
    let res = uc.exec().await;

    response::map(res, None)
}
