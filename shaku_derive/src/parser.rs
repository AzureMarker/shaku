use crate::consts;
use syn::Attribute;

mod key_value;
mod metadata_from_input;
mod module;
mod properties_from_input;
mod property_from_field;

pub(self) use self::key_value::KeyValue;

/// Generic parser for syn structures
// Note: Can't use `std::convert::From` here because we don't want to consume `T`
pub trait Parser<T: Sized> {
    fn parse_as(&self) -> syn::Result<T>;
}

/// Find the #[shaku(...)] attribute
fn get_shaku_attribute(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|a| a.path.is_ident(consts::ATTR_NAME))
}
