use arc_swap::ArcSwap;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use tokio_cron_scheduler::{Job, JobScheduler};
use warp::Filter;

use crate::errors::InitError;
use crate::types::data_set::DataSet;
use crate::types::MetaDataStore;

mod errors;
mod handler;
mod types;

use jemallocator::Jemalloc;

// Jemalloc overwrites the default memory allocator.
// This fixes a performance issue due to threat contention in the MUSL memory allocator.
// https://www.linkedin.com/pulse/testing-alternative-c-memory-allocators-pt-2-musl-mystery-gomes
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

const VRB: bool = true;
const PORT: u16 = 3232;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    dbg_print("Load data");
    let meta_data = run_init()
        .await
        .expect("Failed to run init and failed to download metadata");

    dbg_print("Build meta-data store");
    // ArcSwap hot-swaps data in a multi-threaded runtime.
    // https://docs.rs/arc-swap/1.7.1/arc_swap/index.html
    let store: MetaDataStore = Arc::new(ArcSwap::from_pointee(meta_data.clone()));
    let c = store.clone();
    let with_state = warp::any().map(move || store.clone());

    //  tokio_cron_scheduler
    // https://github.com/mvniekerk/tokio-cron-scheduler
    dbg_print("Build scheduler");
    let scheduler = JobScheduler::new()
        .await
        .expect("Failed to build job scheduler");

    // Run a async update every day at 1 am, EST. (EST = UTC+4)
    //                     sec  min  hour  day   month day of week
    let expression = "0   00    1     *     *     *";
    scheduler
        .add(
            Job::new_async(expression, move |_uuid, _l| {
                let store = c.clone();
                Box::pin(async move {
                    dbg_print("Start update");

                    dbg_print("Re-download data");
                    let meta_data = match run_init().await {
                        Ok(res) => res,
                        Err(e) => {
                            eprint!("Updated Error:");
                            eprint!("Updated Error: {}", e);
                            eprint!("Updated Error:");
                            //  notify someone...
                            return;
                        }
                    };

                    // 1) Use hash from existing metadata to determine if anything has changed
                    dbg_print("Load meta-data hash");
                    let guard = store.deref().load();
                    let hash = guard.hash();

                    // 2) If no change, drop the downloaded metadata & do nothing
                    dbg_print("Check meta-data hash");
                    if meta_data.hash() == hash {
                        drop(meta_data);
                        dbg_print("Hash unchanged; no update needed");
                    } else {
                        // 3) if change, update the store with the new metadata
                        dbg_print("Hash changed run update");
                        store.store(Arc::new(meta_data));
                    }
                    dbg_print("Update complete");
                })
            })
            .expect("Failed to create async update job"),
        )
        .await
        .expect("Failed to add update job to scheduler");

    dbg_print("Start job scheduler");
    scheduler.start().await.expect("Failed to start scheduler");

    dbg_print("Build health route");
    let health_check = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(handler::get_health_handler);

    dbg_print("Build stats route");
    let get_stats = warp::get()
        .and(warp::path("stats"))
        .and(warp::path::end())
        .and(with_state.clone())
        .and_then(handler::get_stats_handler);

    let routes = health_check.or(get_stats);

    print_duration("[main]: Starting server took", &start.elapsed());
    print_start_header_simple("Sample Service", "0.0.0.0:4242/");
    warp::serve(routes).run(([0, 0, 0, 0], PORT)).await;
}

fn dbg_print(s: &str) {
    if VRB {
        println!("[main]: {}", s);
    }
}

async fn run_init() -> Result<DataSet, InitError> {
    Ok(DataSet::default())
}
fn print_duration(msg: &str, elapsed: &Duration) {
    if elapsed.as_millis() > 1000 {
        println!("{} {} sec.", msg, elapsed.as_secs());
    } else {
        println!("{} {} ms.", msg, elapsed.as_millis());
    }
}

fn print_start_header_simple(service_name: &str, service_addr: &str) {
    println!();
    println!("||  {}  ||", service_name);
    println!("==========================================");
    println!("Service on endpoint: {}", service_addr);
    println!("==========================================");
    println!();
}
