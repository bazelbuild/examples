use std::{error, fmt};

/// A cron error
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

/// The kind of cron error that occurred
#[derive(Debug)]
pub enum ErrorKind {
    /// Failed to parse an expression
    Expression(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::Expression(ref expr) => write!(f, "Invalid expression: {}", expr),
        }
    }
}

impl error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}
