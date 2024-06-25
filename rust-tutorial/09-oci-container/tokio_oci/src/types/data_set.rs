use crate::types::stats::Stats;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSet {
    stats: Stats,
    hash: u64,
}

impl DataSet {
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn hash(&self) -> u64 {
        self.hash
    }
}
