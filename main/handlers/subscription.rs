use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use subscription::domain::subscription::SubscriptionId;

use crate::handlers::common::Uninmplemented;
use crate::handlers::context::{with_context, Context};

pub fn routes(ctx: &Arc<Context>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_by_id = warp::get()
        .and(warp::path!(SubscriptionId))
        .and(with_context(ctx.clone()))
        .and_then(get_by_id);

    warp::path("subscriptions").and(get_by_id)
}

pub async fn get_by_id(_id: SubscriptionId, _ctx: Arc<Context>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Uninmplemented::new()))
}
