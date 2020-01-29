use std::error::Error as StdError;
use std::fmt::{self, Debug};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error while resolving a component
    ResolveError(String),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::ResolveError(msg) => msg,
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ResolveError(msg) => write!(f, "Resolve Error > {}", msg),
        }
    }
}
