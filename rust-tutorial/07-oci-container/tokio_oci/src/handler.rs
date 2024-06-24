use crate::types::health::Health;
use crate::types::MetaDataStore;
use warp;

pub(crate) async fn get_health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let result = Health::ok();
    Ok(warp::reply::json(&result))
}

pub(crate) async fn get_stats_handler(
    store: MetaDataStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    let guard = store.load();
    let result = guard.stats();
    Ok(warp::reply::json(result))
}
