//! Error type for tinyexpr crate.

use std::error::Error;
use std::fmt;
use std::result;

/// Result type used throughout the crate.
pub type Result<T> = result::Result<T, TinyExprError>;

/// Error type for codespawn crate.
#[derive(Debug)]
pub enum TinyExprError {
    /// Any other kind of error
    Other(String)
}

impl fmt::Display for TinyExprError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TinyExprError::Other(ref err) => err.fmt(f)
        }
    }
}

impl Error for TinyExprError {
    fn description(&self) -> &str {
        match *self {
            TinyExprError::Other(ref err) => err
        }
    }
}

impl From<String> for TinyExprError {
    fn from(err: String) -> TinyExprError {
        TinyExprError::Other(err)
    }
}
