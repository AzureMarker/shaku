//! Generic error type

use std::error::Error as StdError;
use std::fmt::{self, Debug, Display, Formatter};

/// This type represents all possible errors that can occur when registering or resolving components or when generating the code to do so.
#[derive(Clone)]
pub enum Error {
    /// Error generated during procedural macro `#[derive(Component)]`'s code generation.
    ExtractError(String),
    /// Error generated during procedural macro `#[derive(Component)]`'s code generation.
    ParseError(String),
    /// Error possibly received when calling any of the `resolve` method of [Container](struct.Container.html#method.resolve).
    ResolveError(String),
    /// Simple, unqualified error. Not used as part of this crate.
    Basic(String),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ExtractError(ref msg) => msg.as_str(),
            Error::ParseError(ref msg) => msg.as_str(),
            Error::ResolveError(ref msg) => msg.as_str(),
            Error::Basic(ref message) => message.as_str(),
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}

/// Returns the error's `description()` prefixed by the error's type.
/// For RegistrationError, list the message of each the errors encountered (i.e. the 3rd String tuple entry).
impl fmt::Display for Error {
    // Allow calling deprecated function StdError::description
    #[allow(deprecated)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ExtractError(_) => {
                f.write_str(format!("Extract Error > {}", self.description()).as_str())
            }
            Error::ParseError(_) => {
                f.write_str(format!("Parse Error > {}", self.description()).as_str())
            }
            Error::ResolveError(_) => {
                f.write_str(format!("Resolve Error > {}", self.description()).as_str())
            }
            Error::Basic(_) => {
                f.write_str(format!("Basic Error > {}", self.description()).as_str())
            }
        }
    }
}

/// Same as Display.
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(&self, f)
    }
}
