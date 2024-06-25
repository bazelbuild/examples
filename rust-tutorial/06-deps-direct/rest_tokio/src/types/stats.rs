use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    download_timestamp: String,
    hash: String,
    number_assets: u32,
    number_exchanges: u32,
    number_instruments: u32,
}
