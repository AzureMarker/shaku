use std::error::Error as StdError;
use std::fmt::{self, Debug};

#[derive(Clone, Debug)]
pub enum Error {
    /// Error while parsing the input tokens
    ParseError(String),
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError(msg) => write!(f, "{}", msg),
        }
    }
}
