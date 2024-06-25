use crate::types::data_set::DataSet;
use arc_swap::ArcSwap;
use std::sync::Arc;

pub(crate) mod health;
pub(crate) mod data_set;
pub(crate) mod stats;

pub(crate) type MetaDataStore = Arc<ArcSwap<DataSet>>;
