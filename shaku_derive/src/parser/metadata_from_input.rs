use crate::consts;
use crate::error::Error;
use crate::parser::{get_shaku_attribute, KeyValue, Parser};
use crate::structures::MetaData;
use syn::{DeriveInput, Path};

impl Parser<MetaData> for DeriveInput {
    fn parse_as(&self) -> Result<MetaData, Error> {
        // Find the shaku(interface = ?) attribute
        let shaku_attribute = get_shaku_attribute(&self.attrs)?;

        if let syn::AttrStyle::Inner(_) = shaku_attribute.style {
            return Err(Error::ParseError(format!(
                "invalid attribute format > '{:?}' can't be an inner attribute ",
                shaku_attribute
            )));
        }

        // Get the interface key/value
        let path_kv: KeyValue<Path> = shaku_attribute.parse_args().map_err(|_| {
            Error::ParseError(format!(
                "invalid attribute format > '{:?}' the name of the trait must be in name-value form. \
                Example: #[{}({} = <your trait>)]",
                shaku_attribute,
                consts::ATTR_NAME,
                consts::INTERFACE_ATTR_NAME
            ))
        })?;

        if path_kv.key != consts::INTERFACE_ATTR_NAME {
            return Err(Error::ParseError(format!(
                "unable to find interface > please add a '#[{}({} = <your trait>)]'",
                consts::ATTR_NAME,
                consts::INTERFACE_ATTR_NAME
            )));
        }

        Ok(MetaData {
            identifier: self.ident.clone(),
            interface: path_kv.value.get_ident().unwrap().clone(),
        })
    }
}
