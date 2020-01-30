use crate::error::Error;

mod metadata_from_input;
mod path_key_value;
mod properties_from_input;
mod property_from_field;

pub use self::path_key_value::PathKeyValue;

/// Generic parser for syn structures
// Note: Can't use `std::convert::From` here because we don't want to consume `T`
pub trait Parser<T: Sized> {
    fn parse_as(&self) -> Result<T, Error>;
}
