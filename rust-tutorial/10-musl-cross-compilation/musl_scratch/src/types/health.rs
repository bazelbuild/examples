use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Health {
    status: String,
}

impl Health {
    pub fn ok() -> Self {
        Self {
            status: String::from("OK"),
        }
    }
}
