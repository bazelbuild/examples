// Copyright 2024 The Bazel examples and tutorials Authors & Contributors. // All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


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
