use std::error::Error as StdError;
use std::fmt::{self, Debug};

/// Alias for a `Result` with the error type [shaku::Error](enum.Error.html)
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Error while resolving a component
    ResolveError(String),
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ResolveError(msg) => write!(f, "Resolve Error: {}", msg),
        }
    }
}
