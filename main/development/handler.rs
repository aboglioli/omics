use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use crate::container::{with_container, Container};

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /categories
    let development = warp::get()
        .and(warp::path::end())
        .and(with_container(container.clone()))
        .and_then(development);

    warp::path("development").and(development)
}

pub async fn development(_c: Arc<Container>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::html("<b>Ok</b>"))
}
