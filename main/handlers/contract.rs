use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use contract::domain::contract::ContractId;

use crate::handlers::common::Uninmplemented;
use crate::handlers::container::{with_container, Container};

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_by_id = warp::get()
        .and(warp::path!(ContractId))
        .and(with_container(container.clone()))
        .and_then(get_by_id);

    warp::path("contracts").and(get_by_id)
}

pub async fn get_by_id(
    _id: ContractId,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}