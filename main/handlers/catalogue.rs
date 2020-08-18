use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use catalogue::application::catalogue::Get;

use crate::container::{with_container, Container};
use crate::response;

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /catalogue
    let catalogue = warp::get()
        .and(warp::path::end())
        .and(with_container(container.clone()))
        .and_then(catalogue);

    warp::path("catalogue").and(catalogue)
}

pub async fn catalogue(c: Arc<Container>) -> Result<impl Reply, Rejection> {
    let uc = Get::new(c.catalogue.catalogue_repo());
    let res = uc.exec().await;

    response::map(res, None)
}
