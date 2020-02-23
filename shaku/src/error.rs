use std::error::Error as StdError;
use std::fmt::{self, Debug};

/// Alias for a `Result` with the error type [shaku::Error](enum.Error.html)
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Error while registering a component/provider
    Registration(String),
    /// Error while resolving a component
    ResolveError(String),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::Registration(msg) => msg,
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
            Error::Registration(msg) => write!(f, "Registration Error: {}", msg),
            Error::ResolveError(msg) => write!(f, "Resolve Error: {}", msg),
        }
    }
}
