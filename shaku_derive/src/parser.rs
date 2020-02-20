use crate::consts;
use crate::error::Error;
use syn::Attribute;

mod key_value;
mod metadata_from_input;
mod properties_from_input;
mod property_from_field;

pub(self) use self::key_value::KeyValue;

/// Generic parser for syn structures
// Note: Can't use `std::convert::From` here because we don't want to consume `T`
pub trait Parser<T: Sized> {
    fn parse_as(&self) -> Result<T, Error>;
}

pub(self) fn get_shaku_attribute(attrs: &[Attribute]) -> Result<&Attribute, Error> {
    attrs
        .iter()
        .find(|a| a.path.is_ident(consts::ATTR_NAME))
        .ok_or_else(|| {
            Error::ParseError(format!(
                "unable to find interface > please add a '#[{}({} = <your trait>)]'",
                consts::ATTR_NAME,
                consts::INTERFACE_ATTR_NAME
            ))
        })
}
