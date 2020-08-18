use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use identity::application::role::GetAll;

use crate::authorization::with_auth;
use crate::container::{with_container, Container};
use crate::response;

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /roles
    let get_all = warp::get()
        .and(warp::path::end())
        .and(with_auth(container.clone()))
        .and(with_container(container.clone()))
        .and_then(get_all);

    warp::path("roles").and(get_all)
}

pub async fn get_all(user_id: String, c: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = GetAll::new(c.identity.role_repo(), c.identity.user_repo());
    let res = uc.exec(user_id).await;

    response::map(res, None)
}
