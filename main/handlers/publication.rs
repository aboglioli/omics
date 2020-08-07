use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use publishing::domain::publication::PublicationId;

use crate::handlers::common::Uninmplemented;
use crate::handlers::container::{with_container, Container};

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_by_id = warp::get()
        .and(warp::path!(PublicationId))
        .and(with_container(container.clone()))
        .and_then(get_by_id);

    // Administration
    let create = warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(create);

    let update = warp::put()
        .and(warp::path!(PublicationId))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(update);

    let delete = warp::delete()
        .and(warp::path!(PublicationId))
        .and(with_container(container.clone()))
        .and_then(delete);

    let publish = warp::post()
        .and(warp::path!(PublicationId))
        .and(with_container(container.clone()))
        .and_then(publish);

    let approve = warp::post()
        .and(warp::path!(PublicationId))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(approve);

    let reject = warp::post()
        .and(warp::path!(PublicationId))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(reject);

    // Interactions
    let read = warp::post()
        .and(warp::path!(PublicationId / "read"))
        .and(with_container(container.clone()))
        .and_then(read);

    let like = warp::post()
        .and(warp::path!(PublicationId / "like"))
        .and(with_container(container.clone()))
        .and_then(like);

    let review = warp::post()
        .and(warp::path!(PublicationId / "review"))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(review);

    warp::path("publications").and(
        get_by_id
            .or(create)
            .or(update)
            .or(delete)
            .or(publish)
            .or(approve)
            .or(reject)
            .or(read)
            .or(like)
            .or(review),
    )
}

pub async fn get_by_id(
    _id: PublicationId,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn create(
    _cmd: Uninmplemented,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn update(
    _id: PublicationId,
    _cmd: Uninmplemented,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn delete(
    _id: PublicationId,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn publish(
    _id: PublicationId,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn approve(
    _id: PublicationId,
    _cmd: Uninmplemented,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn reject(
    _id: PublicationId,
    _cmd: Uninmplemented,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn read(_id: PublicationId, _container: Arc<Container>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn like(_id: PublicationId, _container: Arc<Container>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn review(
    _id: PublicationId,
    _cmd: Uninmplemented,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}
