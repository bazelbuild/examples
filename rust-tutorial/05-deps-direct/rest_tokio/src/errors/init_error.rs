use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct InitError(pub String);

impl From<&str> for InitError {
    fn from(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl From<String> for InitError {
    fn from(field0: String) -> Self {
        Self(field0)
    }
}

impl Display for InitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "InitError: {}", self.0)
    }
}

impl Error for InitError {}
