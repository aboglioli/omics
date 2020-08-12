use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use crate::container::{with_container, Container};
use crate::handlers::common::Uninmplemented;

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_by_id = warp::get()
        .and(warp::path!(String))
        .and(with_container(container.clone()))
        .and_then(get_by_id);

    warp::path("subscriptions").and(get_by_id)
}

pub async fn get_by_id(_id: String, _container: Arc<Container>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}
