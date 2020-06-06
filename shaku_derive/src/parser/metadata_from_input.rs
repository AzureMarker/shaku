use crate::consts;
use crate::error::Error;
use crate::parser::{get_shaku_attribute, KeyValue, Parser};
use crate::structures::service::MetaData;
use syn::{DeriveInput, Type};

impl Parser<MetaData> for DeriveInput {
    fn parse_as(&self) -> Result<MetaData, Error> {
        // Find the shaku(interface = ?) attribute
        let shaku_attribute = get_shaku_attribute(&self.attrs).ok_or_else(|| {
            Error::ParseError(format!(
                "Unable to find interface. Please add a '#[{}({} = <your trait>)]'",
                consts::ATTR_NAME,
                consts::INTERFACE_ATTR_NAME
            ))
        })?;

        // Get the interface key/value
        let interface_kv: KeyValue<Type> = shaku_attribute.parse_args().map_err(|_| {
            Error::ParseError(format!(
                "Invalid attribute format. The attribute must be in name-value form. \
                Example: #[{}({} = <your trait>)]",
                consts::ATTR_NAME,
                consts::INTERFACE_ATTR_NAME
            ))
        })?;

        if interface_kv.key != consts::INTERFACE_ATTR_NAME {
            return Err(Error::ParseError(format!(
                "Unable to find interface. Please add a '#[{}({} = <your trait>)]'",
                consts::ATTR_NAME,
                consts::INTERFACE_ATTR_NAME
            )));
        }

        Ok(MetaData {
            identifier: self.ident.clone(),
            generics: self.generics.clone(),
            interface: interface_kv.value,
            visibility: self.vis.clone(),
        })
    }
}
