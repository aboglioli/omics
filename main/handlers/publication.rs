use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use crate::container::{with_container, Container};
use crate::handlers::common::Uninmplemented;

pub fn routes(
    container: &Arc<Container>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let view = warp::get()
        .and(warp::path!(String))
        .and(with_container(container.clone()))
        .and_then(view);

    // Administration
    let create = warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(create);

    let update = warp::put()
        .and(warp::path!(String))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(update);

    let delete = warp::delete()
        .and(warp::path!(String))
        .and(with_container(container.clone()))
        .and_then(delete);

    let publish = warp::post()
        .and(warp::path!(String))
        .and(with_container(container.clone()))
        .and_then(publish);

    let approve = warp::post()
        .and(warp::path!(String))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(approve);

    let reject = warp::post()
        .and(warp::path!(String))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(reject);

    // Interactions
    let read = warp::post()
        .and(warp::path!(String / "read"))
        .and(with_container(container.clone()))
        .and_then(read);

    let like = warp::post()
        .and(warp::path!(String / "like"))
        .and(with_container(container.clone()))
        .and_then(like);

    let review = warp::post()
        .and(warp::path!(String / "review"))
        .and(warp::body::json())
        .and(with_container(container.clone()))
        .and_then(review);

    warp::path("publications").and(
        view.or(create)
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

pub async fn view(_id: String, _container: Arc<Container>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn create(
    _cmd: Uninmplemented,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn update(
    _id: String,
    _cmd: Uninmplemented,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn delete(_id: String, _container: Arc<Container>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn publish(_id: String, _container: Arc<Container>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn approve(
    _id: String,
    _cmd: Uninmplemented,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn reject(
    _id: String,
    _cmd: Uninmplemented,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn read(_id: String, _container: Arc<Container>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn like(_id: String, _container: Arc<Container>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}

pub async fn review(
    _id: String,
    _cmd: Uninmplemented,
    _container: Arc<Container>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}
