use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use publishing::application::category::GetById;

use crate::authorization::with_auth;
use crate::container::{with_container, Container};
use crate::response;

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /categories/:id
    let get_by_id = warp::get()
        .and(warp::path!(String))
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(get_by_id);

    warp::path("categories").and(get_by_id)
}

pub async fn get_by_id(
    id: String,
    _user_id: String,
    c: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    let uc = GetById::new(c.publishing.category_repo());
    let res = uc.exec(id).await;

    response::map(res, None)
}
