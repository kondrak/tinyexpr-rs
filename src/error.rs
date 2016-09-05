//! Error type for tinyexpr crate.

use std::error::Error;
use std::fmt;
use std::result;
use std::num::ParseFloatError;

/// Result type used throughout the crate.
pub type Result<T> = result::Result<T, TinyExprError>;

/// Error type for codespawn crate.
#[derive(Debug)]
pub enum TinyExprError {
    /// Parse error
    Parse(ParseFloatError),
    /// Any other kind of error
    Other(String)
}

impl fmt::Display for TinyExprError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TinyExprError::Parse(ref err) => err.fmt(f),
            TinyExprError::Other(ref err) => err.fmt(f)
        }
    }
}

impl Error for TinyExprError {
    fn description(&self) -> &str {
        match *self {
            TinyExprError::Parse(ref err) => err.description(),
            TinyExprError::Other(ref err) => err
        }
    }
}

impl From<String> for TinyExprError {
    fn from(err: String) -> TinyExprError {
        TinyExprError::Other(err)
    }
}

impl From<ParseFloatError> for TinyExprError {
    fn from(err: ParseFloatError) -> TinyExprError {
        TinyExprError::Parse(err)
    }
}
